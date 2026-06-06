CREATE TABLE clicks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT NOT NULL,
    user_agent TEXT,
    referer TEXT,
    ip_address TEXT,
    clicked_at TEXT NOT NULL,
    FOREIGN KEY(code) REFERENCES links(code) ON DELETE CASCADE
);

CREATE INDEX idx_clicks_code ON clicks(code);
CREATE INDEX idx_clicks_clicked_at ON clicks(clicked_at);
