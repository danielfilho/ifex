# Publishing Guide for IFEX CLI

## Pre-publication Checklist

1. **Update package.json**:
   - [ ] Set correct `author` field with your name and email
   - [ ] Update `repository.url` with your GitHub repository
   - [ ] Update `homepage` URL
   - [ ] Update `bugs.url` for issue tracking

2. **Test the package**:

   ```bash
   npm test
   npm pack
   node src/index.js --help
   node src/index.js --version
   ```

3. **Check package contents**:

   ```bash
   npm pack --dry-run
   ```

## Publishing Steps

### First-time setup

```bash
npm login
```

### Publish to npm

```bash
# Test package installation locally first
npm pack
npm install -g ./ifex-cli-0.1.0.tgz

# Test global installation
ifex --help

# If everything works, publish
npm publish
```

### Version Updates

```bash
# For patches (bug fixes)
npm version patch

# For minor features
npm version minor

# For major/breaking changes
npm version major

# Then publish
npm publish
```

## Post-publication

1. Create a GitHub release
2. Update documentation if needed
3. Test installation: `npm install -g ifex-cli`

## Package Structure

The published package includes:

- `src/` - All source files
- `README.md` - Documentation
- `LICENSE` - MIT license
- `package.json` - Package metadata

Files excluded via `.npmignore`:

- Development files
- IDE configuration
- Test files
- Documentation drafts
