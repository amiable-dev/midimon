<script>
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

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
  });
</script>

<main>
  <h1>MIDIMon Configuration</h1>

  <div class="status-section">
    <h2>Daemon Status</h2>
    {#if daemonStatus}
      <div class="status-card">
        <p>Running: {daemonStatus.running ? '✅' : '❌'}</p>
        <p>Connected: {daemonStatus.connected ? '✅' : '❌'}</p>
        {#if daemonStatus.error}
          <p class="error">{daemonStatus.error}</p>
        {/if}
      </div>
    {:else if error}
      <p class="error">{error}</p>
    {:else}
      <p>Loading...</p>
    {/if}
  </div>

  <div class="placeholder">
    <p>Visual configuration interface coming soon...</p>
    <ul>
      <li>MIDI Learn mode (Week 3)</li>
      <li>Config editor (Week 4)</li>
      <li>Per-app profiles (Week 5)</li>
      <li>Device templates & polish (Week 6)</li>
    </ul>
  </div>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #1e1e1e;
    color: #e0e0e0;
  }

  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem;
  }

  h1 {
    color: #4a9eff;
    margin-bottom: 2rem;
  }

  .status-section {
    background: #2a2a2a;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
  }

  .status-card {
    background: #333;
    padding: 1rem;
    border-radius: 4px;
  }

  .status-card p {
    margin: 0.5rem 0;
  }

  .error {
    color: #ff6b6b;
  }

  .placeholder {
    background: #2a2a2a;
    border: 2px dashed #444;
    border-radius: 8px;
    padding: 2rem;
    text-align: center;
  }

  .placeholder ul {
    list-style: none;
    padding: 0;
    margin: 1rem 0;
  }

  .placeholder li {
    margin: 0.5rem 0;
    color: #888;
  }
</style>
