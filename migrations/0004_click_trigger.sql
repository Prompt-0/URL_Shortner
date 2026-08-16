CREATE TRIGGER IF NOT EXISTS increment_clicks_on_insert
AFTER INSERT ON clicks
BEGIN
    UPDATE links SET clicks = clicks + 1 WHERE code = NEW.code;
END;
