<script>
  import { invoke } from '@tauri-apps/api/core';

  let configPath = '';
  let error = null;

  async function loadConfigPath() {
    try {
      configPath = await invoke('get_config_path');
    } catch (err) {
      error = err.toString();
    }
  }

  loadConfigPath();
</script>

<div class="view">
  <header class="view-header">
    <h2>Settings</h2>
    <p class="subtitle">Application and daemon configuration</p>
  </header>

  <div class="content">
    <section class="settings-section">
      <h3>Configuration</h3>
      <div class="setting-group">
        <label for="config-path">Config File Path:</label>
        {#if configPath}
          <input
            id="config-path"
            type="text"
            value={configPath}
            readonly
            class="config-path-input"
          />
        {:else if error}
          <p class="error">{error}</p>
        {:else}
          <p class="loading">Loading...</p>
        {/if}
      </div>
    </section>

    <section class="settings-section">
      <h3>Advanced Settings</h3>
      <div class="placeholder">
        <p>Coming in Week 6 (AMI-181-187)...</p>
        <ul>
          <li>Auto-start on login</li>
          <li>Logging level configuration</li>
          <li>Performance tuning</li>
          <li>Keyboard shortcuts</li>
          <li>Theme selection</li>
        </ul>
      </div>
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

  .settings-section {
    margin-bottom: 2.5rem;
  }

  h3 {
    margin: 0 0 1.25rem;
    font-size: 1.25rem;
    color: #e0e0e0;
  }

  .setting-group {
    margin-bottom: 1.5rem;
  }

  label {
    display: block;
    margin-bottom: 0.5rem;
    font-weight: 500;
    color: #999;
    font-size: 0.9rem;
  }

  .config-path-input {
    width: 100%;
    max-width: 600px;
    padding: 0.75rem;
    background: #2a2a2a;
    border: 1px solid #444;
    border-radius: 4px;
    color: #e0e0e0;
    font-family: 'Monaco', 'Courier New', monospace;
    font-size: 0.85rem;
  }

  .config-path-input:focus {
    outline: none;
    border-color: #4a9eff;
  }

  .error {
    color: #ff6b6b;
    margin: 0;
  }

  .loading {
    color: #999;
    margin: 0;
  }

  .placeholder {
    background: #2a2a2a;
    border: 2px dashed #444;
    border-radius: 8px;
    padding: 2rem;
  }

  .placeholder p {
    color: #999;
    margin: 0 0 1rem;
    font-size: 0.95rem;
  }

  .placeholder ul {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .placeholder li {
    margin: 0.75rem 0;
    color: #666;
    font-size: 0.9rem;
  }
</style>
