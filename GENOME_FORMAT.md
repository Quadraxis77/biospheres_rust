# Genome File Format

BioSpheres genomes can be saved and loaded as human-readable JSON files for easy sharing and version control.

## File Format

Genomes are stored as JSON files with the following structure:

```json
{
  "name": "Genome Name",
  "initial_mode": 0,
  "initial_orientation": {
    "x": 0.0,
    "y": 0.0,
    "z": 0.0,
    "w": 1.0
  },
  "modes": [
    {
      "name": "Mode Name",
      "color": { "x": 1.0, "y": 1.0, "z": 1.0 },
      "cell_type": 0,
      "split_mass": 1.0,
      "split_interval": 5.0,
      "max_adhesions": 20,
      "min_adhesions": 0
    }
  ]
}
```

See the reference project for complete field descriptions.
