CREATE TRIGGER update_package_versions_updated_at BEFORE UPDATE
ON package_versions FOR EACH ROW EXECUTE PROCEDURE 
update_updated_at_column();

CREATE TRIGGER update_packages_updated_at BEFORE UPDATE
ON packages FOR EACH ROW EXECUTE PROCEDURE 
update_updated_at_column();
