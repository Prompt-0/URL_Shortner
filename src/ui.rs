pub const HOME_HTML: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Antigravity URL Shortener</title>
  <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600;800&display=swap" rel="stylesheet">
  <style>
    :root {
      --bg-color: #0b0f19;
      --text-main: #f8fafc;
      --text-muted: #94a3b8;
      --accent: #3b82f6;
      --glass-bg: rgba(255, 255, 255, 0.03);
      --glass-border: rgba(255, 255, 255, 0.08);
    }
    body {
      font-family: 'Outfit', sans-serif;
      background-color: var(--bg-color);
      color: var(--text-main);
      margin: 0;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      background-image: 
        radial-gradient(circle at 15% 50%, rgba(59, 130, 246, 0.15), transparent 25%),
        radial-gradient(circle at 85% 30%, rgba(147, 51, 234, 0.15), transparent 25%);
    }
    .container {
      width: 100%;
      max-width: 600px;
      padding: 2rem;
      box-sizing: border-box;
      animation: fadeIn 0.8s ease-out;
    }
    h1 {
      font-size: 3.5rem;
      font-weight: 800;
      text-align: center;
      margin-bottom: 0.5rem;
      background: linear-gradient(135deg, #60a5fa, #c084fc);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
    p.subtitle {
      text-align: center;
      color: var(--text-muted);
      margin-bottom: 3rem;
      font-size: 1.1rem;
    }
    .glass-card {
      background: var(--glass-bg);
      backdrop-filter: blur(12px);
      -webkit-backdrop-filter: blur(12px);
      border: 1px solid var(--glass-border);
      border-radius: 24px;
      padding: 2.5rem;
      box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
      transition: transform 0.3s ease, box-shadow 0.3s ease;
    }
    .glass-card:hover {
      transform: translateY(-5px);
      box-shadow: 0 30px 60px -12px rgba(0, 0, 0, 0.6);
    }
    .form-group {
      margin-bottom: 1.5rem;
    }
    .helper-text {
      margin-top: 0.5rem;
      font-size: 0.85rem;
      color: var(--text-muted);
    }
    label {
      display: block;
      margin-bottom: 0.5rem;
      color: var(--text-muted);
      font-weight: 600;
      font-size: 0.9rem;
      text-transform: uppercase;
      letter-spacing: 0.05em;
    }
    input {
      width: 100%;
      padding: 1rem 1.2rem;
      border-radius: 12px;
      border: 1px solid var(--glass-border);
      background: rgba(0, 0, 0, 0.2);
      color: var(--text-main);
      font-family: 'Outfit', sans-serif;
      font-size: 1rem;
      box-sizing: border-box;
      transition: all 0.3s ease;
    }
    input:focus {
      outline: none;
      border-color: var(--accent);
      box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.2);
      background: rgba(0, 0, 0, 0.4);
    }
    button {
      width: 100%;
      padding: 1rem;
      border: none;
      border-radius: 12px;
      background: linear-gradient(135deg, #3b82f6, #8b5cf6);
      color: white;
      font-family: 'Outfit', sans-serif;
      font-size: 1.1rem;
      font-weight: 600;
      cursor: pointer;
      transition: all 0.3s ease;
      margin-top: 1rem;
    }
    button:hover {
      transform: translateY(-2px);
      box-shadow: 0 10px 20px -10px rgba(139, 92, 246, 0.5);
      background: linear-gradient(135deg, #4f46e5, #7c3aed);
    }
    button:focus-visible {
      outline: none;
      box-shadow: 0 0 0 3px rgba(139, 92, 246, 0.5);
    }
    @keyframes fadeIn {
      from { opacity: 0; transform: translateY(20px); }
      to { opacity: 1; transform: translateY(0); }
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Shorten.</h1>
    <p class="subtitle">The world's fastest, most aesthetic URL shortener.</p>
    
    <div class="glass-card">
      <form action="/shorten" method="post">
        <div class="form-group">
          <label for="url">Long URL <span style="color: #ef4444;" aria-hidden="true">*</span></label>
          <input id="url" name="url" type="url" placeholder="https://example.com/very-long-url" required>
        </div>
        <div class="form-group">
          <label for="custom_code">Custom Alias (Optional)</label>
          <input id="custom_code" name="custom_code" type="text" placeholder="e.g. my-link" pattern="[a-zA-Z0-9\-_]+" minlength="3" maxlength="32" title="Must be 3-32 characters, using only letters, numbers, hyphens, and underscores" aria-describedby="custom_code_help">
          <div id="custom_code_help" class="helper-text">3-32 characters: letters, numbers, hyphens, underscores.</div>
        </div>
        <button type="submit">Create Short Link</button>
      </form>
    </div>
  </div>
</body>
</html>
"#;

pub const SUCCESS_HTML_TEMPLATE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Link Created</title>
  <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600;800&display=swap" rel="stylesheet">
  <style>
    :root {
      --bg-color: #0b0f19;
      --text-main: #f8fafc;
      --text-muted: #94a3b8;
      --glass-bg: rgba(255, 255, 255, 0.03);
      --glass-border: rgba(255, 255, 255, 0.08);
    }
    body {
      font-family: 'Outfit', sans-serif;
      background-color: var(--bg-color);
      color: var(--text-main);
      margin: 0;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      background-image: 
        radial-gradient(circle at 50% 0%, rgba(16, 185, 129, 0.15), transparent 35%);
    }
    .container {
      width: 100%;
      max-width: 600px;
      padding: 2rem;
      box-sizing: border-box;
      animation: fadeIn 0.6s ease-out;
    }
    h1 {
      font-size: 2.5rem;
      font-weight: 800;
      text-align: center;
      margin-bottom: 2rem;
      background: linear-gradient(135deg, #34d399, #10b981);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
    .glass-card {
      background: var(--glass-bg);
      backdrop-filter: blur(12px);
      -webkit-backdrop-filter: blur(12px);
      border: 1px solid var(--glass-border);
      border-radius: 24px;
      padding: 2.5rem;
      box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
    }
    .stat-row {
      margin-bottom: 1.5rem;
      padding-bottom: 1.5rem;
      border-bottom: 1px solid var(--glass-border);
    }
    .stat-row:last-child {
      margin-bottom: 0;
      padding-bottom: 0;
      border-bottom: none;
    }
    .label {
      display: block;
      color: var(--text-muted);
      font-size: 0.9rem;
      text-transform: uppercase;
      letter-spacing: 0.05em;
      margin-bottom: 0.5rem;
    }
    .value a {
      color: #60a5fa;
      text-decoration: none;
      font-weight: 600;
      font-size: 1.2rem;
      word-break: break-all;
      transition: color 0.2s;
    }
    .value a:hover {
      color: #93c5fd;
    }
    .value a:focus-visible {
      outline: none;
      box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.5);
      border-radius: 4px;
    }
    .value code {
      background: rgba(0, 0, 0, 0.3);
      padding: 0.4rem 0.8rem;
      border-radius: 8px;
      font-family: monospace;
      color: #e2e8f0;
      word-break: break-all;
    }
    .actions {
      margin-top: 2rem;
      text-align: center;
    }
    .actions a {
      display: inline-block;
      padding: 0.8rem 2rem;
      border-radius: 12px;
      background: rgba(255, 255, 255, 0.1);
      color: white;
      text-decoration: none;
      font-weight: 600;
      transition: all 0.3s ease;
    }
    .actions a:hover {
      background: rgba(255, 255, 255, 0.2);
      transform: translateY(-2px);
    }
    .actions a:focus-visible {
      outline: none;
      box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.3);
    }
    @keyframes fadeIn {
      from { opacity: 0; transform: scale(0.95); }
      to { opacity: 1; transform: scale(1); }
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Ready to share!</h1>
    <div class="glass-card">
      <div class="stat-row">
        <span class="label">Short URL</span>
        <div class="value"><a href="{short_url}" target="_blank">{short_url}</a></div>
      </div>
      <div class="stat-row">
        <span class="label">Analytics Dashboard</span>
        <div class="value"><a href="{stats_url}">{stats_url}</a></div>
      </div>
      <div class="stat-row">
        <span class="label">Original Destination</span>
        <div class="value"><code>{original_url}</code></div>
      </div>
      <div class="stat-row" style="text-align: center; padding-top: 1.5rem;">
        <span class="label">QR Code</span>
        <img src="/qr/{code}" alt="QR Code" style="border-radius: 12px; margin-top: 1rem; width: 150px; height: 150px; box-shadow: 0 4px 6px rgba(0,0,0,0.1);">
      </div>
    </div>
    <div class="actions">
      <a href="/">Create Another</a>
    </div>
  </div>
</body>
</html>
"#;

pub const STATS_HTML_TEMPLATE: &str = r#"
<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Analytics | {code}</title>
  <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600;800&display=swap" rel="stylesheet">
  <style>
    :root {
      --bg-color: #0b0f19;
      --text-main: #f8fafc;
      --text-muted: #94a3b8;
      --glass-bg: rgba(255, 255, 255, 0.03);
      --glass-border: rgba(255, 255, 255, 0.08);
    }
    body {
      font-family: 'Outfit', sans-serif;
      background-color: var(--bg-color);
      color: var(--text-main);
      margin: 0;
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: center;
      background-image: 
        radial-gradient(circle at 80% 80%, rgba(244, 63, 94, 0.15), transparent 35%);
    }
    .container {
      width: 100%;
      max-width: 600px;
      padding: 2rem;
      box-sizing: border-box;
      animation: fadeIn 0.6s ease-out;
    }
    h1 {
      font-size: 2.5rem;
      font-weight: 800;
      text-align: center;
      margin-bottom: 0.5rem;
      background: linear-gradient(135deg, #fb7185, #e11d48);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
    p.subtitle {
      text-align: center;
      color: var(--text-muted);
      margin-bottom: 2rem;
    }
    .glass-card {
      background: var(--glass-bg);
      backdrop-filter: blur(12px);
      -webkit-backdrop-filter: blur(12px);
      border: 1px solid var(--glass-border);
      border-radius: 24px;
      padding: 2.5rem;
      box-shadow: 0 25px 50px -12px rgba(0, 0, 0, 0.5);
    }
    .metrics-grid {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 1.5rem;
      margin-bottom: 2rem;
    }
    .metric-box {
      background: rgba(0, 0, 0, 0.2);
      border: 1px solid var(--glass-border);
      border-radius: 16px;
      padding: 1.5rem;
      text-align: center;
    }
    .metric-value {
      font-size: 2.5rem;
      font-weight: 800;
      color: white;
      margin-bottom: 0.5rem;
    }
    .metric-label {
      color: var(--text-muted);
      font-size: 0.9rem;
      text-transform: uppercase;
      letter-spacing: 0.05em;
    }
    .details {
      border-top: 1px solid var(--glass-border);
      padding-top: 1.5rem;
    }
    .detail-row {
      display: flex;
      justify-content: space-between;
      margin-bottom: 1rem;
      font-size: 0.95rem;
    }
    .detail-label {
      color: var(--text-muted);
    }
    .detail-value {
      color: white;
      font-family: monospace;
      background: rgba(0,0,0,0.3);
      padding: 2px 6px;
      border-radius: 4px;
    }
    .detail-value a { color: #60a5fa; text-decoration: none; }
    .detail-value a:focus-visible {
      outline: none;
      box-shadow: 0 0 0 3px rgba(96, 165, 250, 0.5);
      border-radius: 4px;
    }
    .actions {
      margin-top: 2rem;
      text-align: center;
    }
    .actions a {
      display: inline-block;
      padding: 0.8rem 2rem;
      border-radius: 12px;
      background: rgba(255, 255, 255, 0.1);
      color: white;
      text-decoration: none;
      font-weight: 600;
      transition: all 0.3s ease;
    }
    .actions a:hover {
      background: rgba(255, 255, 255, 0.2);
      transform: translateY(-2px);
    }
    .actions a:focus-visible {
      outline: none;
      box-shadow: 0 0 0 3px rgba(255, 255, 255, 0.3);
    }
    @keyframes fadeIn {
      from { opacity: 0; transform: translateY(10px); }
      to { opacity: 1; transform: translateY(0); }
    }
  </style>
</head>
<body>
  <div class="container">
    <h1>Analytics</h1>
    <p class="subtitle">Performance for /{code}</p>
    <div class="glass-card">
      <div class="metrics-grid">
        <div class="metric-box">
          <div class="metric-value">{clicks}</div>
          <div class="metric-label">Total Clicks</div>
        </div>
        <div class="metric-box">
          <div class="metric-value" style="font-size: 1.5rem; padding-top: 0.5rem;">Active</div>
          <div class="metric-label">Status</div>
        </div>
      </div>
      <div class="details">
        <div class="detail-row">
          <span class="detail-label">Short URL</span>
          <span class="detail-value"><a href="{short_url}" target="_blank">{short_url}</a></span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Original URL</span>
          <span class="detail-value" style="max-width: 200px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">{original_url}</span>
        </div>
        <div class="detail-row">
          <span class="detail-label">Created At</span>
          <span class="detail-value">{created_at}</span>
        </div>
      </div>
    </div>
    <div class="actions">
      <a href="/">Back to Home</a>
    </div>
  </div>
</body>
</html>
"#;
