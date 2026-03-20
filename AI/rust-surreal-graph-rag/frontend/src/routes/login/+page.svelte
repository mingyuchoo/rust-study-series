<script lang="ts">
  import { login } from '$lib/stores/auth';
  import { goto } from '$app/navigation';

  let email = $state('');
  let password = $state('');
  let loading = $state(false);
  let error = $state<string | null>(null);

  async function onSubmit() {
    loading = true;
    error = null;
    try {
      await login({ email, password });
      goto('/chat');
    } catch (e: any) {
      error = e?.message ?? '로그인에 실패했습니다.';
    } finally {
      loading = false;
    }
  }
</script>

<!-- 로그인 페이지 -->
<div class="login-page">
  <div class="login-card card">
    <div class="stack">
      <div class="login-header">
        <h2>로그인</h2>
        <p>계속하려면 계정 정보를 입력하세요</p>
      </div>

      <div class="field">
        <label for="email">이메일</label>
        <input id="email" type="text" placeholder="name@example.com" bind:value={email} />
      </div>

      <div class="field">
        <label for="password">비밀번호</label>
        <input id="password" type="password" placeholder="비밀번호 입력" bind:value={password} />
      </div>

      {#if error}
        <div class="error">{error}</div>
      {/if}

      <button class="btn btn-primary btn-full" onclick={onSubmit} disabled={loading}>
        {loading ? '로그인 중...' : '로그인'}
      </button>
    </div>
  </div>
</div>

<style>
  .login-page {
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: calc(100vh - 56px);
    padding: 24px;
  }

  .login-card {
    width: 100%;
    max-width: 400px;
  }

  .login-header {
    margin-bottom: 8px;
  }

  .login-header h2 {
    font-size: 22px;
    font-weight: 700;
    letter-spacing: -0.03em;
    color: var(--color-black);
    margin-bottom: 4px;
  }

  .login-header p {
    font-size: 13px;
    color: var(--color-gray-500);
  }

  :global(.btn-full) {
    width: 100%;
  }
</style>
