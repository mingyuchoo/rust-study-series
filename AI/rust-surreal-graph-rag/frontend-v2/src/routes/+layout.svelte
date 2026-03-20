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
  <div class="nav-links">
    <a href="/">홈</a>
    <a href="/chat">채팅</a>
    <a href="/vector-search">벡터검색</a>
    <a href="/graph-search">그래프검색</a>
    <a href="/reindex">재인덱싱</a>
    <a href="/health">헬스</a>
  </div>
  <div class="nav-auth">
    {#if $isAuthenticated}
      <span class="user-email">{$meStore?.email}</span>
      <button class="btn btn-primary" onclick={onLogout}>로그아웃</button>
    {:else}
      <button class="btn btn-default" onclick={() => goto('/login')}>로그인</button>
    {/if}
  </div>
</nav>

<!-- 인증 필요한 페이지 접근 제어 -->
{#if protectedPaths.some((p) => page.url.pathname.startsWith(p)) && !$isAuthenticated}
  <div class="page">
    <p>이 페이지에 접근하려면 <a href="/login">로그인</a>이 필요합니다.</p>
  </div>
{:else}
  {@render children()}
{/if}

<style>
  .navbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px;
    border-bottom: 1px solid var(--color-border);
    margin-bottom: 12px;
  }

  .nav-links {
    display: flex;
    gap: 8px;
  }

  .nav-auth {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .user-email {
    color: var(--color-text-secondary);
  }
</style>
