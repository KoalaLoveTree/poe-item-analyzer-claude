# Data Directory

This directory contains timeless jewel lookup table (LUT) data files required for analysis.

## Automatic Download (Recommended)

The application will automatically download required data files on first run. Simply:
1. Launch the application
2. Click "Download & Install Data" when prompted
3. Wait for the download to complete

## Manual Download

If automatic download fails, you can manually download the data files:

1. Visit the source repository: https://github.com/Regisle/TimelessJewelData
2. Download the following files from the `data/` directory:
   - `lethal_pride.bin`
   - `brutal_restraint.bin`
   - `glorious_vanity.bin`
   - `elegant_hubris.bin`
   - `militant_faith.bin`
   - `Node_Indices.csv`
   - `Jewel_Node_Link.json`
   - `alternate_passive_additions.json`
   - `alternate_passive_skills.json`
3. Place all files in this directory
4. Restart the application

## File Descriptions

See `manifest.json` for detailed information about each file, including:
- Source URLs
- File sizes
- Checksums (for validation)
- Descriptions

## Data Updates

When a new Path of Exile patch is released, you may need to update the data files:
1. Open the application
2. Go to Settings → Data Management
3. Click "Check for Updates" or "Re-import"

## Storage Location

By default, data files are stored in OS-specific application data directories:
- **Linux**: `~/.local/share/poe-item-analyzer-claude/data/`
- **Windows**: `C:\Users\<user>\AppData\Roaming\poe-item-analyzer-claude\data\`
- **macOS**: `~/Library/Application Support/poe-item-analyzer-claude/data/`

This directory is used during development and for fallback storage.
