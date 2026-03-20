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
<div class="login-container">
  <div class="stack">
    <h2>로그인</h2>

    <div class="field">
      <label for="email">이메일</label>
      <input id="email" type="text" bind:value={email} />
    </div>

    <div class="field">
      <label for="password">비밀번호</label>
      <input id="password" type="password" bind:value={password} />
    </div>

    {#if error}
      <div class="error">{error}</div>
    {/if}

    <button class="btn btn-primary" onclick={onSubmit} disabled={loading}>
      {loading ? '로그인 중...' : '로그인'}
    </button>
  </div>
</div>

<style>
  .login-container {
    max-width: 420px;
    margin: 40px auto;
    padding: 0 16px;
  }
</style>
