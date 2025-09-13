<script lang="ts">
  // Placeholder for Dashboard Page - will be implemented in task 9
</script>

<div class="dashboard-page">
  <div class="placeholder-content">
    <h2>Dashboard Page</h2>
    <p>This page will contain the system health and metrics.</p>
    <p><em>To be implemented in task 9.</em></p>
  </div>
</div>

<style>
  .dashboard-page {
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .placeholder-content {
    text-align: center;
    padding: 2rem;
    background-color: var(--color-surface-100);
    border-radius: 0.5rem;
    border: 2px dashed var(--color-surface-300);
  }

  .placeholder-content h2 {
    margin: 0 0 1rem 0;
    color: var(--color-surface-700);
  }

  .placeholder-content p {
    margin: 0.5rem 0;
    color: var(--color-surface-600);
  }

  .placeholder-content em {
    color: var(--color-surface-500);
    font-size: 0.875rem;
  }

  /* Dark mode support */
  @media (prefers-color-scheme: dark) {
    .placeholder-content {
      background-color: var(--color-surface-800);
      border-color: var(--color-surface-600);
    }

    .placeholder-content h2 {
      color: var(--color-surface-300);
    }

    .placeholder-content p {
      color: var(--color-surface-400);
    }

    .placeholder-content em {
      color: var(--color-surface-500);
    }
  }
</style>