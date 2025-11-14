<script>
  // Mappings view - Week 3-4
</script>

<div class="view">
  <header class="view-header">
    <h2>Mappings</h2>
    <p class="subtitle">Configure MIDI event to action mappings</p>
  </header>

  <div class="content">
    <div class="placeholder">
      <h3>Mapping Editor</h3>
      <p>Coming in Week 3-4 (AMI-167-176)...</p>
      <ul>
        <li>Visual mapping editor</li>
        <li>MIDI Learn mode (Week 3)</li>
        <li>Trigger configuration (velocity, long-press, chords)</li>
        <li>Action configuration (keystroke, launch, shell)</li>
        <li>Drag-and-drop mapping organization</li>
      </ul>
    </div>
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

  .placeholder {
    background: #2a2a2a;
    border: 2px dashed #444;
    border-radius: 8px;
    padding: 3rem 2rem;
    text-align: center;
  }

  .placeholder h3 {
    color: #4a9eff;
    margin: 0 0 0.5rem;
    font-size: 1.5rem;
  }

  .placeholder p {
    color: #999;
    margin: 0.5rem 0 1.5rem;
    font-size: 1rem;
  }

  .placeholder ul {
    list-style: none;
    padding: 0;
    margin: 1rem 0 0;
  }

  .placeholder li {
    margin: 0.75rem 0;
    color: #666;
    font-size: 0.95rem;
  }
</style>
