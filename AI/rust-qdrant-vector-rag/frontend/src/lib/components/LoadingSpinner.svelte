<script lang="ts">
  import { Brain, Search, FileText } from 'lucide-svelte';
  import { onMount } from 'svelte';

  // Props
  export let size: 'xs' | 'sm' | 'md' | 'lg' | 'xl' = 'md';
  export let message = 'AI is thinking...';
  export let showProgress = false;
  export let progress = 0;
  export let variant: 'search' | 'upload' | 'processing' = 'search';

  // Animation state
  let animationPhase = 0;
  let mounted = false;

  // Animation messages for different phases
  const searchMessages = [
    'AI is thinking...',
    'Analyzing your question...',
    'Searching through documents...',
    'Finding relevant information...',
    'Generating response...',
    'Crafting the perfect answer...',
    'Almost ready...',
  ];

  const uploadMessages = [
    'Processing document...',
    'Extracting text content...',
    'Creating vector embeddings...',
    'Storing in database...',
    'Finalizing upload...',
    'Optimizing for search...',
    'Nearly complete...',
  ];

  const processingMessages = [
    'Processing...',
    'Working on it...',
    'Almost done...',
    'Finishing up...',
    'Just a moment...',
    'Wrapping up...',
  ];

  // Get messages based on variant
  $: messages =
    variant === 'search'
      ? searchMessages
      : variant === 'upload'
        ? uploadMessages
        : processingMessages;

  // Get icon based on variant
  $: IconComponent = variant === 'search' ? Brain : variant === 'upload' ? FileText : Search;

  // Animate through different messages
  onMount(() => {
    mounted = true;
    const interval = setInterval(() => {
      animationPhase = (animationPhase + 1) % messages.length;
    }, 2000);

    return () => clearInterval(interval);
  });

  // Current message based on animation phase or prop
  $: currentMessage = mounted && messages.length > 1 ? messages[animationPhase] : message;

  // Size mappings for different elements
  $: loaderSize =
    size === 'xs' ? 16 : size === 'sm' ? 20 : size === 'md' ? 24 : size === 'lg' ? 32 : 40;

  $: iconSize =
    size === 'xs' ? 16 : size === 'sm' ? 20 : size === 'md' ? 24 : size === 'lg' ? 28 : 32;

  $: textSize =
    size === 'xs'
      ? 'xs'
      : size === 'sm'
        ? 'sm'
        : size === 'md'
          ? 'md'
          : size === 'lg'
            ? 'lg'
            : 'xl';
</script>

<div
  class="loading-spinner"
  class:compact={size === 'xs' || size === 'sm'}
  role="status"
  aria-live="polite"
>
  <div class="flex flex-col items-center space-y-4">
    <!-- Main loading indicator -->
    <div class="spinner-container">
      <div class="icon-background">
        <svelte:component this={IconComponent} size={iconSize} class="variant-icon" />
      </div>
      <div class="spinner-dots">
        <div class="dot"></div>
        <div class="dot"></div>
        <div class="dot"></div>
      </div>
    </div>

    <!-- Loading message -->
    <div class="message-container">
      <p
        class="text-center font-medium"
        class:text-sm={textSize === 'xs' || textSize === 'sm'}
        class:text-base={textSize === 'md'}
        class:text-lg={textSize === 'lg'}
        class:text-xl={textSize === 'xl'}
        aria-label={currentMessage}
      >
        {currentMessage}
      </p>
    </div>

    <!-- Progress bar (optional) -->
    {#if showProgress}
      <div class="progress-container w-full max-w-xs">
        <div class="w-full bg-gray-200 rounded-full h-2 dark:bg-gray-700">
          <div
            class="bg-blue-600 h-2 rounded-full transition-all duration-300 ease-out"
            style="width: {progress}%"
            aria-label="Progress: {progress}%"
          ></div>
        </div>
        <p class="text-xs text-gray-600 text-center mt-1">
          {Math.round(progress)}%
        </p>
      </div>
    {/if}
  </div>
</div>

<style>
  .loading-spinner {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    min-height: 120px;
  }

  .loading-spinner.compact {
    padding: 1rem;
    min-height: 80px;
  }

  .spinner-container {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .icon-background {
    position: absolute;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgb(187 222 251); /* primary-100 */
    border-radius: 50%;
    padding: 0.5rem;
    z-index: 1;
  }

  .icon-background :global(.variant-icon) {
    color: rgb(30 136 229); /* primary-600 */
    animation: pulse 2s ease-in-out infinite;
  }

  .spinner-dots {
    display: flex;
    gap: 0.25rem;
    margin-left: 3rem;
  }

  .dot {
    width: 8px;
    height: 8px;
    background-color: rgb(30 136 229); /* primary-600 */
    border-radius: 50%;
    animation: bounce 1.4s ease-in-out infinite both;
  }

  .dot:nth-child(1) {
    animation-delay: -0.32s;
  }
  .dot:nth-child(2) {
    animation-delay: -0.16s;
  }
  .dot:nth-child(3) {
    animation-delay: 0s;
  }

  .message-container {
    max-width: 300px;
    text-align: center;
  }

  .progress-container {
    width: 100%;
    max-width: 200px;
  }

  /* Animations */
  @keyframes pulse {
    0%,
    100% {
      opacity: 0.7;
      transform: scale(1);
    }
    50% {
      opacity: 1;
      transform: scale(1.05);
    }
  }

  @keyframes bounce {
    0%,
    80%,
    100% {
      transform: scale(0);
    }
    40% {
      transform: scale(1);
    }
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .icon-background {
      background: rgb(13 71 161); /* primary-900 */
    }

    .icon-background :global(.variant-icon) {
      color: rgb(66 165 245); /* primary-400 */
    }

    .dot {
      background-color: rgb(66 165 245); /* primary-400 */
    }
  }

  /* Responsive design */
  @media (max-width: 480px) {
    .loading-spinner {
      padding: 1rem;
    }

    .message-container {
      max-width: 250px;
    }

    .progress-container {
      max-width: 180px;
    }
  }

  /* Accessibility improvements */
  @media (prefers-reduced-motion: reduce) {
    .icon-background :global(.variant-icon) {
      animation: none;
    }

    .dot {
      animation: none;
    }
  }

  /* High contrast mode support */
  @media (prefers-contrast: high) {
    .icon-background {
      border: 2px solid rgb(30 136 229); /* primary-600 */
    }
  }
</style>
