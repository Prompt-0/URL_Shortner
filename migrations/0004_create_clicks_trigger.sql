CREATE TRIGGER IF NOT EXISTS trigger_clicks_insert
AFTER INSERT ON clicks
BEGIN
    UPDATE links SET clicks = clicks + 1 WHERE code = NEW.code;
END;
