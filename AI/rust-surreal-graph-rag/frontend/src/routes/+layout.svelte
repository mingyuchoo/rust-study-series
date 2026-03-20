<script lang="ts">
  import { onMount } from 'svelte';
  import { initAuth, isAuthenticated, meStore, logout } from '$lib/stores/auth';
  import { goto } from '$app/navigation';
  import { page } from '$app/state';
  import '../app.css';

  let { children } = $props();

  onMount(() => {
    initAuth();
  });

  async function onLogout() {
    await logout();
    goto('/');
  }

  // 인증이 필요한 경로
  const protectedPaths = ['/chat', '/vector-search', '/graph-search', '/reindex'];
</script>

<!-- 상단 네비게이션 바 -->
<nav class="navbar">
  <div class="nav-left">
    <a href="/" class="nav-brand">RAG</a>
    <div class="nav-links">
      <a href="/chat" class="nav-link" class:active={page.url.pathname === '/chat'}>채팅</a>
      <a href="/vector-search" class="nav-link" class:active={page.url.pathname === '/vector-search'}>벡터검색</a>
      <a href="/graph-search" class="nav-link" class:active={page.url.pathname === '/graph-search'}>그래프검색</a>
      <a href="/reindex" class="nav-link" class:active={page.url.pathname === '/reindex'}>재인덱싱</a>
      <a href="/health" class="nav-link" class:active={page.url.pathname === '/health'}>헬스</a>
    </div>
  </div>
  <div class="nav-auth">
    {#if $isAuthenticated}
      <span class="user-email">{$meStore?.email}</span>
      <button class="btn btn-default btn-sm" onclick={onLogout}>로그아웃</button>
    {:else}
      <button class="btn btn-primary btn-sm" onclick={() => goto('/login')}>로그인</button>
    {/if}
  </div>
</nav>

<!-- 인증 필요한 페이지 접근 제어 -->
{#if protectedPaths.some((p) => page.url.pathname.startsWith(p)) && !$isAuthenticated}
  <div class="page">
    <div class="auth-guard">
      <h2>인증 필요</h2>
      <p>이 페이지에 접근하려면 로그인이 필요합니다.</p>
      <button class="btn btn-primary" onclick={() => goto('/login')}>로그인하기</button>
    </div>
  </div>
{:else}
  {@render children()}
{/if}

<style>
  .navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 24px;
    height: 56px;
    background: var(--color-white);
    border-bottom: 1px solid var(--color-gray-200);
    position: sticky;
    top: 0;
    z-index: 100;
    backdrop-filter: blur(8px);
    background: rgba(255, 255, 255, 0.9);
  }

  .nav-left {
    display: flex;
    align-items: center;
    gap: 32px;
  }

  .nav-brand {
    font-size: 16px;
    font-weight: 700;
    letter-spacing: -0.04em;
    color: var(--color-black);
  }

  .nav-brand:hover {
    color: var(--color-black);
  }

  .nav-links {
    display: flex;
    gap: 4px;
  }

  .nav-link {
    padding: 6px 12px;
    border-radius: var(--radius-sm);
    font-size: 13px;
    font-weight: 500;
    color: var(--color-gray-600);
    transition:
      color var(--transition),
      background-color var(--transition);
  }

  .nav-link:hover {
    color: var(--color-gray-900);
    background-color: var(--color-gray-100);
    text-decoration: none;
  }

  .nav-link.active {
    color: var(--color-gray-900);
    background-color: var(--color-gray-100);
  }

  .nav-auth {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .user-email {
    font-size: 13px;
    color: var(--color-gray-500);
  }

  :global(.btn-sm) {
    padding: 5px 14px;
    font-size: 12px;
  }

  .auth-guard {
    text-align: center;
    padding: 80px 0;
  }

  .auth-guard p {
    color: var(--color-gray-500);
    margin-bottom: 24px;
  }
</style>
