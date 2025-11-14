<script>
  import { statusStore } from '../stores.js';
  import DeviceList from '../components/DeviceList.svelte';
</script>

<div class="view">
  <header class="view-header">
    <h2>Devices</h2>
    <p class="subtitle">Manage MIDI device connections</p>
  </header>

  <div class="content">
    <section class="status-section">
      <h3>Daemon Status</h3>
      {#if $statusStore.status}
        <div class="status-card">
          <div class="status-row">
            <span class="label">Running:</span>
            <span class="value">{$statusStore.status.running ? '✅ Yes' : '❌ No'}</span>
          </div>
          <div class="status-row">
            <span class="label">Connected:</span>
            <span class="value">{$statusStore.status.connected ? '✅ Yes' : '❌ No'}</span>
          </div>
          {#if $statusStore.status.lifecycle_state}
            <div class="status-row">
              <span class="label">State:</span>
              <span class="value">{$statusStore.status.lifecycle_state}</span>
            </div>
          {/if}
          {#if $statusStore.status.uptime_secs !== null && $statusStore.status.uptime_secs !== undefined}
            <div class="status-row">
              <span class="label">Uptime:</span>
              <span class="value">{Math.floor($statusStore.status.uptime_secs / 60)}m {$statusStore.status.uptime_secs % 60}s</span>
            </div>
          {/if}
          {#if $statusStore.status.error}
            <div class="status-row error">
              <span class="label">Error:</span>
              <span class="value">{$statusStore.status.error}</span>
            </div>
          {/if}
        </div>
      {:else if $statusStore.error}
        <p class="error">{$statusStore.error}</p>
      {:else}
        <p class="loading">Loading...</p>
      {/if}
    </section>

    <section class="devices-section">
      <DeviceList />
    </section>
  </div>
</div>

<style>
  .view {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .view-header {
    padding: 2rem 2.5rem 1.5rem;
    border-bottom: 1px solid #333;
    background: #1e1e1e;
  }

  .view-header h2 {
    margin: 0 0 0.5rem;
    font-size: 1.75rem;
    color: #e0e0e0;
  }

  .subtitle {
    margin: 0;
    color: #999;
    font-size: 0.95rem;
  }

  .content {
    flex: 1;
    padding: 2rem 2.5rem;
    overflow-y: auto;
  }

  .status-section,
  .devices-section {
    margin-bottom: 2rem;
  }

  h3 {
    margin: 0 0 1rem;
    font-size: 1.25rem;
    color: #e0e0e0;
  }

  .status-card {
    background: #2a2a2a;
    border-radius: 8px;
    padding: 1.5rem;
    border: 1px solid #333;
  }

  .status-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 0;
    border-bottom: 1px solid #333;
  }

  .status-row:last-child {
    border-bottom: none;
  }

  .status-row.error {
    color: #ff6b6b;
  }

  .label {
    font-weight: 500;
    color: #999;
  }

  .value {
    color: #e0e0e0;
    font-weight: 500;
  }

  .error {
    color: #ff6b6b;
  }

  .loading {
    color: #999;
  }
</style>
