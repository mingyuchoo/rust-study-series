<script lang="ts">
  import { currentPage, isOnline } from '../stores/app.store.js';
  import { appActions } from '../stores/app.store.js';
  import { Upload, Search, Activity, Menu, X } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { BreakpointUtils } from '../utils/responsive.js';
  import { FocusManager, KeyboardNavigation, generateId } from '../utils/accessibility.js';

  let isMobileMenuOpen = false;
  let isMobile = false;
  let navElement: HTMLElement;
  let mobileMenuButton: HTMLButtonElement;
  let currentFocusIndex = -1;
  
  // Generate unique IDs for accessibility
  const menuId = generateId('nav-menu');
  const menuButtonId = generateId('nav-menu-button');

  // Navigation items with enhanced accessibility
  const navItems = [
    { 
      id: 'upload', 
      label: 'Upload Documents', 
      icon: Upload, 
      path: '/upload',
      description: 'Upload PDF documents to the system'
    },
    { 
      id: 'search', 
      label: 'Search Documents', 
      icon: Search, 
      path: '/search',
      description: 'Search and ask questions about uploaded documents'
    },
    { 
      id: 'dashboard', 
      label: 'System Dashboard', 
      icon: Activity, 
      path: '/dashboard',
      description: 'View system health and status'
    }
  ];

  // Handle navigation with accessibility announcements
  function navigateTo(pageId: string, announce = true) {
    const item = navItems.find(item => item.id === pageId);
    appActions.setCurrentPage(pageId);
    isMobileMenuOpen = false;
    
    if (announce && item) {
      // Announce page change to screen readers
      const announcement = document.createElement('div');
      announcement.setAttribute('aria-live', 'polite');
      announcement.setAttribute('aria-atomic', 'true');
      announcement.className = 'sr-only';
      announcement.textContent = `Navigated to ${item.label}`;
      document.body.appendChild(announcement);
      
      setTimeout(() => {
        document.body.removeChild(announcement);
      }, 1000);
    }
  }

  // Toggle mobile menu with focus management
  function toggleMobileMenu() {
    isMobileMenuOpen = !isMobileMenuOpen;
    
    if (isMobileMenuOpen) {
      // Focus first menu item when opening
      setTimeout(() => {
        const firstMenuItem = navElement?.querySelector('[role="menuitem"]') as HTMLElement;
        firstMenuItem?.focus();
        currentFocusIndex = 0;
      }, 100);
    } else {
      // Return focus to menu button when closing
      mobileMenuButton?.focus();
      currentFocusIndex = -1;
    }
  }

  // Handle keyboard navigation in mobile menu
  function handleMenuKeydown(event: KeyboardEvent) {
    if (!isMobileMenuOpen) return;

    const menuItems = Array.from(navElement?.querySelectorAll('[role="menuitem"]') || []) as HTMLElement[];
    
    switch (event.key) {
      case KeyboardNavigation.KEYS.ESCAPE:
        event.preventDefault();
        toggleMobileMenu();
        break;
        
      case KeyboardNavigation.KEYS.ARROW_DOWN:
        event.preventDefault();
        currentFocusIndex = (currentFocusIndex + 1) % menuItems.length;
        menuItems[currentFocusIndex]?.focus();
        break;
        
      case KeyboardNavigation.KEYS.ARROW_UP:
        event.preventDefault();
        currentFocusIndex = currentFocusIndex === 0 ? menuItems.length - 1 : currentFocusIndex - 1;
        menuItems[currentFocusIndex]?.focus();
        break;
        
      case KeyboardNavigation.KEYS.HOME:
        event.preventDefault();
        currentFocusIndex = 0;
        menuItems[0]?.focus();
        break;
        
      case KeyboardNavigation.KEYS.END:
        event.preventDefault();
        currentFocusIndex = menuItems.length - 1;
        menuItems[currentFocusIndex]?.focus();
        break;
    }
  }

  // Handle menu item activation
  function handleMenuItemClick(pageId: string) {
    navigateTo(pageId);
  }

  function handleMenuItemKeydown(event: KeyboardEvent, pageId: string) {
    if (KeyboardNavigation.isActivationKey(event.key)) {
      event.preventDefault();
      navigateTo(pageId);
    }
  }

  // Check if mobile on mount and resize with enhanced breakpoint detection
  onMount(() => {
    const cleanup = BreakpointUtils.onBreakpointChange((breakpoint) => {
      const wasMobile = isMobile;
      isMobile = breakpoint === 'mobile';
      
      // Close mobile menu when switching to desktop
      if (wasMobile && !isMobile && isMobileMenuOpen) {
        isMobileMenuOpen = false;
        currentFocusIndex = -1;
      }
    });

    // Handle clicks outside mobile menu
    const handleClickOutside = (event: MouseEvent) => {
      if (isMobileMenuOpen && navElement && !navElement.contains(event.target as Node)) {
        isMobileMenuOpen = false;
        currentFocusIndex = -1;
      }
    };

    document.addEventListener('click', handleClickOutside);

    return () => {
      cleanup();
      document.removeEventListener('click', handleClickOutside);
    };
  });
</script>

<!-- Skip link for keyboard navigation -->
<a href="#main-content" class="skip-link">Skip to main content</a>

<nav 
  bind:this={navElement}
  class="navigation" 
  class:mobile={isMobile}

  aria-label="Main navigation"
>
  <!-- Navigation Header -->
  <div class="nav-header">
    <div class="nav-brand">
      <h1 class="brand-title" id="app-title">
        <span class="sr-only">RAG Document Search Application</span>
        <span aria-hidden="true">RAG Search</span>
      </h1>
      <div 
        class="status-indicator" 
        class:online={$isOnline} 
        class:offline={!$isOnline}
        role="status"
        aria-live="polite"
        aria-label={$isOnline ? 'Application is online' : 'Application is offline'}
      >
        <span class="status-dot" aria-hidden="true"></span>
        <span class="status-text">{$isOnline ? 'Online' : 'Offline'}</span>
      </div>
    </div>

    <!-- Mobile menu toggle -->
    {#if isMobile}
      <button 
        bind:this={mobileMenuButton}
        id={menuButtonId}
        class="mobile-menu-toggle min-touch-target focus-visible"
        on:click={toggleMobileMenu}
        aria-label={isMobileMenuOpen ? 'Close navigation menu' : 'Open navigation menu'}
        aria-expanded={isMobileMenuOpen}
        aria-controls={menuId}
        aria-haspopup="true"
      >
        {#if isMobileMenuOpen}
          <X size={24} aria-hidden="true" />
          <span class="sr-only">Close menu</span>
        {:else}
          <Menu size={24} aria-hidden="true" />
          <span class="sr-only">Open menu</span>
        {/if}
      </button>
    {/if}
  </div>

  <!-- Navigation Menu -->
  <div 
    id={menuId}
    class="nav-menu" 
    class:open={isMobileMenuOpen}
    role={isMobile ? 'menu' : 'navigation'}
    aria-labelledby={isMobile ? menuButtonId : 'app-title'}
    aria-hidden={isMobile ? !isMobileMenuOpen : false}
    on:keydown={handleMenuKeydown}
  >
    <ul class="nav-list" role={isMobile ? 'none' : 'list'}>
      {#each navItems as item, index}
        <li class="nav-item" role="none">
          <button
            class="nav-link min-touch-target focus-visible"
            class:active={$currentPage === item.id}
            on:click={() => handleMenuItemClick(item.id)}
            on:keydown={(e) => handleMenuItemKeydown(e, item.id)}
            role={isMobile ? 'menuitem' : 'button'}
            aria-current={$currentPage === item.id ? 'page' : undefined}
            aria-describedby={`nav-desc-${item.id}`}
            tabindex={isMobile && !isMobileMenuOpen ? -1 : 0}
          >
            <svelte:component 
              this={item.icon} 
              size={20} 
              class="nav-icon" 
              aria-hidden="true"
            />
            <span class="nav-label">{item.label}</span>
            <span id={`nav-desc-${item.id}`} class="sr-only">
              {item.description}
            </span>
          </button>
        </li>
      {/each}
    </ul>
  </div>
</nav>

<style>
  /* Skip link styles */
  :global(.skip-link) {
    position: absolute;
    top: -40px;
    left: var(--spacing-sm);
    background: var(--color-primary-600);
    color: white;
    padding: var(--spacing-xs) var(--spacing-sm);
    text-decoration: none;
    border-radius: 0.375rem;
    z-index: var(--z-modal);
    font-weight: 600;
    font-size: var(--font-size-sm);
    transition: top var(--duration-fast) ease;
  }

  :global(.skip-link:focus) {
    top: var(--spacing-sm);
  }

  /* Navigation container */
  .navigation {
    background-color: var(--color-surface-100);
    border-bottom: 1px solid var(--color-surface-300);
    position: relative;
    z-index: var(--z-sticky);
    box-shadow: var(--shadow-sm);
  }

  /* Navigation header */
  .nav-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-md);
    min-height: 80px;
  }

  .nav-brand {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .brand-title {
    margin: 0;
    font-size: var(--font-size-xl);
    font-weight: 700;
    color: var(--color-primary-600);
    line-height: var(--line-height-tight);
  }

  /* Status indicator */
  .status-indicator {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    font-size: var(--font-size-sm);
    padding: var(--spacing-xs) 0;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    transition: background-color var(--duration-fast) ease;
    flex-shrink: 0;
  }

  .status-indicator.online .status-dot {
    background-color: var(--color-success-500);
    box-shadow: 0 0 0 2px var(--color-success-100);
  }

  .status-indicator.offline .status-dot {
    background-color: var(--color-error-500);
    box-shadow: 0 0 0 2px var(--color-error-100);
  }

  .status-text {
    color: var(--color-surface-600);
    font-weight: 500;
  }

  /* Mobile menu toggle */
  .mobile-menu-toggle {
    display: none;
    background: none;
    border: 1px solid var(--color-surface-300);
    cursor: pointer;
    padding: var(--spacing-sm);
    border-radius: 0.5rem;
    color: var(--color-surface-700);
    transition: all var(--duration-fast) ease;
    position: relative;
  }

  .mobile-menu-toggle:hover {
    background-color: var(--color-surface-200);
    border-color: var(--color-surface-400);
  }

  .mobile-menu-toggle:active {
    transform: scale(0.98);
  }

  /* Navigation menu */
  .nav-menu {
    padding: 0 var(--spacing-md) var(--spacing-md);
  }

  .nav-list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    gap: var(--spacing-xs);
  }

  .nav-item {
    flex: 1;
  }

  /* Navigation links */
  .nav-link {
    display: flex;
    align-items: center;
    gap: var(--spacing-sm);
    padding: var(--spacing-sm) var(--spacing-md);
    background: none;
    border: 1px solid transparent;
    border-radius: 0.5rem;
    cursor: pointer;
    transition: all var(--duration-fast) ease;
    color: var(--color-surface-700);
    font-size: var(--font-size-sm);
    font-weight: 500;
    width: 100%;
    text-align: left;
    position: relative;
  }

  .nav-link:hover {
    background-color: var(--color-surface-200);
    color: var(--color-surface-900);
    border-color: var(--color-surface-300);
    transform: translateY(-1px);
    box-shadow: var(--shadow-sm);
  }

  .nav-link.active {
    background-color: var(--color-primary-100);
    color: var(--color-primary-700);
    border-color: var(--color-primary-200);
    box-shadow: var(--shadow-sm);
  }

  .nav-link.active :global(.nav-icon) {
    color: var(--color-primary-600);
  }

  .nav-link:active {
    transform: translateY(0);
  }

  :global(.nav-icon) {
    flex-shrink: 0;
    transition: color var(--duration-fast) ease;
  }

  .nav-label {
    white-space: nowrap;
    font-weight: 500;
  }

  /* Mobile styles */
  .navigation.mobile {
    border-bottom: none;
  }

  .navigation.mobile .mobile-menu-toggle {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .navigation.mobile .nav-menu {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    background-color: var(--color-surface-100);
    border-bottom: 1px solid var(--color-surface-300);
    box-shadow: var(--shadow-lg);
    transform: translateY(-100%);
    opacity: 0;
    visibility: hidden;
    transition: all var(--duration-normal) cubic-bezier(0.4, 0, 0.2, 1);
    padding: var(--spacing-md);
    z-index: var(--z-dropdown);
  }

  .navigation.mobile .nav-menu.open {
    transform: translateY(0);
    opacity: 1;
    visibility: visible;
  }

  .navigation.mobile .nav-list {
    flex-direction: column;
    gap: var(--spacing-xs);
  }

  .navigation.mobile .nav-item {
    flex: none;
  }

  .navigation.mobile .nav-link {
    justify-content: flex-start;
    padding: var(--spacing-md);
    border-radius: 0.5rem;
  }

  /* Tablet styles */
  @media (min-width: 768px) and (max-width: 1023px) {
    .nav-header {
      padding: var(--spacing-lg);
    }
    
    .nav-menu {
      padding: 0 var(--spacing-lg) var(--spacing-lg);
    }
    
    .nav-link {
      padding: var(--spacing-md) var(--spacing-lg);
      font-size: var(--font-size-base);
    }
  }

  /* Desktop styles */
  @media (min-width: 1024px) {
    .navigation {
      width: 320px;
      height: 100vh;
      border-right: 1px solid var(--color-surface-300);
      border-bottom: none;
      display: flex;
      flex-direction: column;
      position: sticky;
      top: 0;
    }

    .nav-header {
      border-bottom: 1px solid var(--color-surface-300);
      flex-shrink: 0;
      padding: var(--spacing-xl);
    }

    .nav-brand {
      align-items: flex-start;
    }

    .brand-title {
      font-size: var(--font-size-2xl);
    }

    .nav-menu {
      flex: 1;
      padding: var(--spacing-xl) var(--spacing-lg);
      overflow-y: auto;
    }

    .nav-list {
      flex-direction: column;
      gap: var(--spacing-sm);
    }

    .nav-link {
      justify-content: flex-start;
      padding: var(--spacing-md) var(--spacing-lg);
      font-size: var(--font-size-base);
      border-radius: 0.75rem;
    }

    .nav-link:hover {
      transform: translateX(4px);
    }
  }

  /* Large desktop styles */
  @media (min-width: 1440px) {
    .navigation {
      width: 360px;
    }
    
    .nav-header {
      padding: var(--spacing-2xl) var(--spacing-xl);
    }
    
    .nav-menu {
      padding: var(--spacing-2xl) var(--spacing-xl);
    }
    
    .nav-link {
      padding: var(--spacing-lg) var(--spacing-xl);
      gap: var(--spacing-md);
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

    .status-indicator.online .status-dot {
      box-shadow: 0 0 0 2px var(--color-surface-700);
    }

    .status-indicator.offline .status-dot {
      box-shadow: 0 0 0 2px var(--color-surface-700);
    }

    .mobile-menu-toggle {
      color: var(--color-surface-300);
      border-color: var(--color-surface-600);
    }

    .mobile-menu-toggle:hover {
      background-color: var(--color-surface-700);
      border-color: var(--color-surface-500);
    }

    .nav-link {
      color: var(--color-surface-300);
    }

    .nav-link:hover {
      background-color: var(--color-surface-700);
      color: var(--color-surface-100);
      border-color: var(--color-surface-600);
    }

    .nav-link.active {
      background-color: var(--color-primary-900);
      color: var(--color-primary-300);
      border-color: var(--color-primary-800);
    }

    .navigation.mobile .nav-menu {
      background-color: var(--color-surface-800);
      border-color: var(--color-surface-700);
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .navigation {
      border-width: 2px;
    }
    
    .nav-link {
      border-width: 2px;
    }
    
    .nav-link.active {
      border-width: 3px;
    }
    
    .mobile-menu-toggle {
      border-width: 2px;
    }
  }

  /* Reduced motion support */
  @media (prefers-reduced-motion: reduce) {
    .mobile-menu-toggle,
    .nav-link,
    .status-dot,
    .nav-menu {
      transition: none;
    }
    
    .nav-link:hover {
      transform: none;
    }
    
    .mobile-menu-toggle:active {
      transform: none;
    }
  }

  /* Print styles */
  @media print {
    .navigation {
      display: none;
    }
  }
</style>