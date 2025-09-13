<script lang="ts">
  import { currentPage, isOnline } from '../stores/app.store.js';
  import { appActions } from '../stores/app.store.js';
  import { Upload, Search, Activity, Menu, X } from 'lucide-svelte';
  import { onMount } from 'svelte';

  let isMobileMenuOpen = false;
  let isMobile = false;

  // Navigation items
  const navItems = [
    { id: 'upload', label: 'Upload', icon: Upload, path: '/upload' },
    { id: 'search', label: 'Search', icon: Search, path: '/search' },
    { id: 'dashboard', label: 'Dashboard', icon: Activity, path: '/dashboard' }
  ];

  // Handle navigation
  function navigateTo(pageId: string) {
    appActions.setCurrentPage(pageId);
    isMobileMenuOpen = false;
  }

  // Toggle mobile menu
  function toggleMobileMenu() {
    isMobileMenuOpen = !isMobileMenuOpen;
  }

  // Check if mobile on mount and resize
  onMount(() => {
    const checkMobile = () => {
      isMobile = window.innerWidth < 768;
      if (!isMobile) {
        isMobileMenuOpen = false;
      }
    };

    checkMobile();
    window.addEventListener('resize', checkMobile);

    return () => {
      window.removeEventListener('resize', checkMobile);
    };
  });
</script>

<nav class="navigation" class:mobile={isMobile}>
  <!-- Desktop Navigation -->
  <div class="nav-header">
    <div class="nav-brand">
      <h1 class="brand-title">RAG Search</h1>
      <div class="status-indicator" class:online={$isOnline} class:offline={!$isOnline}>
        <span class="status-dot"></span>
        <span class="status-text">{$isOnline ? 'Online' : 'Offline'}</span>
      </div>
    </div>

    <!-- Mobile menu toggle -->
    {#if isMobile}
      <button 
        class="mobile-menu-toggle"
        on:click={toggleMobileMenu}
        aria-label="Toggle navigation menu"
        aria-expanded={isMobileMenuOpen}
      >
        {#if isMobileMenuOpen}
          <X size={24} />
        {:else}
          <Menu size={24} />
        {/if}
      </button>
    {/if}
  </div>

  <!-- Navigation Menu -->
  <div class="nav-menu" class:open={isMobileMenuOpen}>
    <ul class="nav-list" role="menubar">
      {#each navItems as item}
        <li class="nav-item" role="none">
          <button
            class="nav-link"
            class:active={$currentPage === item.id}
            on:click={() => navigateTo(item.id)}
            role="menuitem"
            aria-current={$currentPage === item.id ? 'page' : undefined}
          >
            <svelte:component this={item.icon} size={20} class="nav-icon" />
            <span class="nav-label">{item.label}</span>
          </button>
        </li>
      {/each}
    </ul>
  </div>
</nav>

<style>
  .navigation {
    background-color: var(--color-surface-100);
    border-bottom: 1px solid var(--color-surface-300);
    position: relative;
    z-index: 1000;
  }

  .nav-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
  }

  .nav-brand {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .brand-title {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 700;
    color: var(--color-primary-600);
  }

  .status-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.875rem;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    transition: background-color 0.2s ease;
  }

  .status-indicator.online .status-dot {
    background-color: var(--color-success-500);
  }

  .status-indicator.offline .status-dot {
    background-color: var(--color-error-500);
  }

  .status-text {
    color: var(--color-surface-600);
  }

  .mobile-menu-toggle {
    display: none;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 0.375rem;
    color: var(--color-surface-700);
    transition: background-color 0.2s ease;
  }

  .mobile-menu-toggle:hover {
    background-color: var(--color-surface-200);
  }

  .nav-menu {
    padding: 0 1rem 1rem;
  }

  .nav-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    gap: 0.5rem;
  }

  .nav-item {
    flex: 1;
  }

  .nav-link {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.75rem 1rem;
    background: none;
    border: none;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all 0.2s ease;
    color: var(--color-surface-700);
    font-size: 0.875rem;
    font-weight: 500;
    width: 100%;
    text-align: left;
  }

  .nav-link:hover {
    background-color: var(--color-surface-200);
    color: var(--color-surface-900);
  }

  .nav-link.active {
    background-color: var(--color-primary-100);
    color: var(--color-primary-700);
  }

  .nav-link.active :global(.nav-icon) {
    color: var(--color-primary-600);
  }

  :global(.nav-icon) {
    flex-shrink: 0;
  }

  .nav-label {
    white-space: nowrap;
  }

  /* Mobile styles */
  .navigation.mobile {
    border-bottom: none;
  }

  .navigation.mobile .mobile-menu-toggle {
    display: block;
  }

  .navigation.mobile .nav-menu {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background-color: var(--color-surface-100);
    border-bottom: 1px solid var(--color-surface-300);
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
    transform: translateY(-100%);
    opacity: 0;
    visibility: hidden;
    transition: all 0.3s ease;
    padding: 1rem;
  }

  .navigation.mobile .nav-menu.open {
    transform: translateY(0);
    opacity: 1;
    visibility: visible;
  }

  .navigation.mobile .nav-list {
    flex-direction: column;
    gap: 0.25rem;
  }

  .navigation.mobile .nav-item {
    flex: none;
  }

  /* Desktop styles */
  @media (min-width: 768px) {
    .navigation {
      width: 280px;
      height: 100vh;
      border-right: 1px solid var(--color-surface-300);
      border-bottom: none;
      display: flex;
      flex-direction: column;
    }

    .nav-header {
      border-bottom: 1px solid var(--color-surface-300);
      flex-shrink: 0;
    }

    .nav-brand {
      align-items: flex-start;
    }

    .nav-menu {
      flex: 1;
      padding: 1.5rem 1rem;
    }

    .nav-list {
      flex-direction: column;
      gap: 0.5rem;
    }

    .nav-link {
      justify-content: flex-start;
      padding: 1rem;
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .navigation {
      background-color: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }

    .brand-title {
      color: var(--color-primary-400);
    }

    .status-text {
      color: var(--color-surface-300);
    }

    .mobile-menu-toggle {
      color: var(--color-surface-300);
    }

    .mobile-menu-toggle:hover {
      background-color: var(--color-surface-700);
    }

    .nav-link {
      color: var(--color-surface-300);
    }

    .nav-link:hover {
      background-color: var(--color-surface-700);
      color: var(--color-surface-100);
    }

    .nav-link.active {
      background-color: var(--color-primary-900);
      color: var(--color-primary-300);
    }

    .navigation.mobile .nav-menu {
      background-color: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }
  }

  /* Focus styles for accessibility */
  .nav-link:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }

  .mobile-menu-toggle:focus {
    outline: 2px solid var(--color-primary-500);
    outline-offset: 2px;
  }
</style>