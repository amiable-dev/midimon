<script>
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import DeviceList from '../components/DeviceList.svelte';

  let daemonStatus = null;
  let error = null;

  async function checkStatus() {
    try {
      daemonStatus = await invoke('get_daemon_status');
    } catch (err) {
      error = err.toString();
    }
  }

  onMount(() => {
    checkStatus();
    // Poll every 3 seconds
    const interval = setInterval(checkStatus, 3000);
    return () => clearInterval(interval);
  });
</script>

<div class="view">
  <header class="view-header">
    <h2>Devices</h2>
    <p class="subtitle">Manage MIDI device connections</p>
  </header>

  <div class="content">
    <section class="status-section">
      <h3>Daemon Status</h3>
      {#if daemonStatus}
        <div class="status-card">
          <div class="status-row">
            <span class="label">Running:</span>
            <span class="value">{daemonStatus.running ? '✅ Yes' : '❌ No'}</span>
          </div>
          <div class="status-row">
            <span class="label">Connected:</span>
            <span class="value">{daemonStatus.connected ? '✅ Yes' : '❌ No'}</span>
          </div>
          {#if daemonStatus.lifecycle_state}
            <div class="status-row">
              <span class="label">State:</span>
              <span class="value">{daemonStatus.lifecycle_state}</span>
            </div>
          {/if}
          {#if daemonStatus.uptime_secs !== null && daemonStatus.uptime_secs !== undefined}
            <div class="status-row">
              <span class="label">Uptime:</span>
              <span class="value">{Math.floor(daemonStatus.uptime_secs / 60)}m {daemonStatus.uptime_secs % 60}s</span>
            </div>
          {/if}
          {#if daemonStatus.error}
            <div class="status-row error">
              <span class="label">Error:</span>
              <span class="value">{daemonStatus.error}</span>
            </div>
          {/if}
        </div>
      {:else if error}
        <p class="error">{error}</p>
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
