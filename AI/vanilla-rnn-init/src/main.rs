use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

// 타입 별칭
type Vector = Vec<f64>;
type Matrix = Vec<Vec<f64>>;

#[derive(Debug, Clone)]
struct RNNParams {
    wxh: Matrix, // input -> hidden
    whh: Matrix, // hidden -> hidden (recurrent)
    why: Matrix, // hidden -> output
    bh: Vector,  // hidden bias
    by: Vector,  // output bias
}

#[derive(Debug, Clone)]
struct RNNState {
    hidden_state: Vector,
}

// ============= 행렬/벡터 연산 =============

fn mat_vec_mul(m: &Matrix, v: &Vector) -> Vector { m.iter().map(|row| row.iter().zip(v.iter()).map(|(a, b)| a * b).sum()).collect() }

fn vec_add(a: &Vector, b: &Vector) -> Vector { a.iter().zip(b.iter()).map(|(x, y)| x + y).collect() }

fn vec_sub(a: &Vector, b: &Vector) -> Vector { a.iter().zip(b.iter()).map(|(x, y)| x - y).collect() }

fn vec_scale(s: f64, v: &Vector) -> Vector { v.iter().map(|x| s * x).collect() }

fn outer_product(v1: &Vector, v2: &Vector) -> Matrix { v1.iter().map(|x| v2.iter().map(|y| x * y).collect()).collect() }

fn transpose(m: &Matrix) -> Matrix {
    if m.is_empty() {
        return vec![];
    }
    let cols = m[0].len();
    let rows = m.len();
    let mut t = vec![vec![0.0; rows]; cols];
    for i in 0 .. rows {
        for j in 0 .. cols {
            t[j][i] = m[i][j];
        }
    }
    t
}

fn mat_add(a: &Matrix, b: &Matrix) -> Matrix {
    a.iter()
        .zip(b.iter())
        .map(|(ra, rb)| ra.iter().zip(rb.iter()).map(|(x, y)| x + y).collect())
        .collect()
}

// ============= 활성화 함수 =============

fn tanh_activation(v: &Vector) -> Vector { v.iter().map(|x| x.tanh()).collect() }

fn tanh_derivative(xs: &Vector) -> Vector { xs.iter().map(|x| 1.0 - x * x).collect() }

fn softmax(xs: &Vector) -> Vector {
    let max_x = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let exps: Vec<f64> = xs.iter().map(|x| (x - max_x).exp()).collect();
    let sum_exps: f64 = exps.iter().sum();
    exps.iter().map(|e| e / sum_exps).collect()
}

// ============= RNN 초기화 =============

fn random_matrix(rows: usize, cols: usize, rng: &mut StdRng) -> Matrix {
    let scale = 0.01;
    (0 .. rows)
        .map(|_| (0 .. cols).map(|_| rng.random_range(-1.0 ..= 1.0) * scale).collect())
        .collect()
}

#[allow(dead_code)]
fn random_vector(n: usize, rng: &mut StdRng) -> Vector { (0 .. n).map(|_| rng.random_range(-1.0 ..= 1.0)).collect() }

fn init_rnn(input_size: usize, hidden_size: usize, output_size: usize, rng: &mut StdRng) -> RNNParams {
    let wxh = random_matrix(hidden_size, input_size, rng);
    let whh = random_matrix(hidden_size, hidden_size, rng);
    let why = random_matrix(output_size, hidden_size, rng);
    let bh = vec![0.0; hidden_size]; // Haskell 예시는 randomVector for bh but it's fine to init zeros like many RNNs; to match exactly, use random_vector
    let by = vec![0.0; output_size];
    RNNParams {
        wxh,
        whh,
        why,
        bh,
        by,
    }
}

fn init_hidden_state(hidden_size: usize) -> RNNState {
    RNNState {
        hidden_state: vec![0.0; hidden_size],
    }
}

// ============= RNN Forward Pass =============

fn rnn_step(params: &RNNParams, state: &RNNState, input: &Vector) -> (Vector, RNNState, Vector) {
    let h_prev = &state.hidden_state;
    // h_raw = W_xh * x + W_hh * h_prev + b_h
    let wx = mat_vec_mul(&params.wxh, input);
    let wh = mat_vec_mul(&params.whh, h_prev);
    let h_raw = vec_add(&vec_add(&wx, &wh), &params.bh);
    let h_t = tanh_activation(&h_raw);
    // y_raw = W_hy * h_t + b_y
    let y_raw = vec_add(&mat_vec_mul(&params.why, &h_t), &params.by);
    let y_t = softmax(&y_raw);
    let new_state = RNNState {
        hidden_state: h_t.clone(),
    };
    (y_t, new_state, h_t)
}

fn rnn_forward(params: &RNNParams, init_state: &RNNState, inputs: &[Vector]) -> (Vec<Vector>, Vec<RNNState>) {
    let mut outputs: Vec<Vector> = Vec::new();
    let mut states: Vec<RNNState> = Vec::new();
    let mut state = init_state.clone();
    for x in inputs.iter() {
        let (y, new_state, _h_t) = rnn_step(params, &state, x);
        outputs.push(y);
        states.push(new_state.clone());
        state = new_state;
    }
    (outputs, states)
}

// ============= RNN Backward Pass (단순화된 BPTT) =============

#[derive(Debug, Clone)]
struct RNNGradients {
    dwxh: Matrix,
    dwhh: Matrix,
    dwhy: Matrix,
    dbh: Vector,
    dby: Vector,
}

fn zero_gradients(input_size: usize, hidden_size: usize, output_size: usize) -> RNNGradients {
    RNNGradients {
        dwxh: vec![vec![0.0; input_size]; hidden_size],
        dwhh: vec![vec![0.0; hidden_size]; hidden_size],
        dwhy: vec![vec![0.0; hidden_size]; output_size],
        dbh: vec![0.0; hidden_size],
        dby: vec![0.0; output_size],
    }
}

fn add_gradients(g1: &RNNGradients, g2: &RNNGradients) -> RNNGradients {
    RNNGradients {
        dwxh: mat_add(&g1.dwxh, &g2.dwxh),
        dwhh: mat_add(&g1.dwhh, &g2.dwhh),
        dwhy: mat_add(&g1.dwhy, &g2.dwhy),
        dbh: vec_add(&g1.dbh, &g2.dbh),
        dby: vec_add(&g1.dby, &g2.dby),
    }
}

fn scale_matrix(s: f64, m: &Matrix) -> Matrix { m.iter().map(|row| row.iter().map(|x| x * s).collect()).collect() }

fn update_params(lr: f64, params: &RNNParams, grads: &RNNGradients) -> RNNParams {
    RNNParams {
        wxh: mat_add(&params.wxh, &scale_matrix(-lr, &grads.dwxh)),
        whh: mat_add(&params.whh, &scale_matrix(-lr, &grads.dwhh)),
        why: mat_add(&params.why, &scale_matrix(-lr, &grads.dwhy)),
        bh: vec_add(&params.bh, &vec_scale(-lr, &grads.dbh)),
        by: vec_add(&params.by, &vec_scale(-lr, &grads.dby)),
    }
}

fn cross_entropy_loss(target: &Vector, output: &Vector) -> f64 { -target.iter().zip(output.iter()).map(|(t, o)| t * (o + 1e-8).ln()).sum::<f64>() }

// ---- 간단화된 그래디언트 계산 (Haskell 예시를 그대로 모사) ----

fn compute_step_gradients(params: &RNNParams, input: &Vector, target: &Vector, output: &Vector, h_t: &Vector, dh_next: &Vector) -> RNNGradients {
    // dy = output - target
    let dy = vec_sub(output, target);
    // dh_raw = W_hy^T * dy + dh_next
    let why_t = transpose(&params.why);
    let whyt_dy = mat_vec_mul(&why_t, &dy);
    let dh_raw = vec_add(&whyt_dy, dh_next);
    // dh = dh_raw * tanh_derivative(h_t)
    let dh = {
        let deriv = tanh_derivative(h_t);
        dh_raw.iter().zip(deriv.iter()).map(|(a, b)| a * b).collect()
    };

    RNNGradients {
        dwxh: outer_product(&dh, input),
        // Haskell 예제 used outerProduct dh h_t for dwhh (we follow that)
        dwhh: outer_product(&dh, h_t),
        dwhy: outer_product(&dy, h_t),
        dbh: dh,
        dby: dy,
    }
}

fn compute_simple_gradients(params: &RNNParams, inputs: &[Vector], targets: &[Vector], outputs: &[Vector], states: &[RNNState]) -> RNNGradients {
    let input_size = if !inputs.is_empty() { inputs[0].len() } else { 0 };
    let hidden_size = params.bh.len();
    let output_size = params.by.len();
    let zero_grads = zero_gradients(input_size, hidden_size, output_size);

    // 역순으로 처리하며 그래디언트 누적 (Haskell과 동일한 로직)
    let mut grads_list: Vec<RNNGradients> = Vec::new();
    // dh_next 초기값: zeros
    let mut dh_next = vec![0.0; hidden_size];

    // iterate reversed
    for ((inp, tgt), (out, st)) in inputs.iter().zip(targets.iter()).zip(outputs.iter().zip(states.iter())).rev() {
        let grad = compute_step_gradients(params, inp, tgt, out, &st.hidden_state, &dh_next);
        // dh_next' = transpose(whh) * dbh (dbH is grad.dbh)
        let whh_t = transpose(&params.whh);
        let dh_next_prime = mat_vec_mul(&whh_t, &grad.dbh);
        dh_next = dh_next_prime;
        grads_list.push(grad);
    }

    // fold (sum) gradients
    let mut accum = zero_grads;
    for g in grads_list.into_iter() {
        accum = add_gradients(&accum, &g);
    }
    accum
}

// ============= 학습 예제 =============

fn train_loop(mut params: RNNParams, inputs: &[Vector], targets: &[Vector], lr: f64, epochs: usize) -> RNNParams {
    let hidden_size = params.bh.len();
    for epoch in 1 ..= epochs {
        let (outputs, states) = rnn_forward(&params, &init_hidden_state(hidden_size), inputs);
        let loss: f64 = outputs.iter().zip(targets.iter()).map(|(o, t)| cross_entropy_loss(t, o)).sum();

        let grads = compute_simple_gradients(&params, inputs, targets, &outputs, &states);
        params = update_params(lr, &params, &grads);

        if epoch % 10 == 0 {
            println!("Epoch {}, Loss: {:.6}", epoch, loss);
        }
    }
    params
}

fn train_example() {
    let mut rng = StdRng::from_os_rng();
    let input_size = 3;
    let hidden_size = 5;
    let output_size = 3;
    let learning_rate = 0.1;
    let epochs = 500;

    let mut params = init_rnn(input_size, hidden_size, output_size, &mut rng);

    // 간단한 시퀀스 (Haskell 예시와 동일)
    let inputs: Vec<Vector> = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]];
    let targets: Vec<Vector> = vec![vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0], vec![1.0, 0.0, 0.0]];

    println!("RNN 학습 시작...");
    params = train_loop(params, &inputs, &targets, learning_rate, epochs);

    println!("\n학습된 모델 테스트:");
    let (outputs, _) = rnn_forward(&params, &init_hidden_state(hidden_size), &inputs);
    for i in 0 .. inputs.len() {
        println!("입력: {:?}", inputs[i]);
        println!("예측: {:?}", outputs[i]);
        println!("정답: {:?}", targets[i]);
        println!();
    }
}

fn main() { train_example(); }
