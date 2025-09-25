# @AXIOMHIVE @DEVDOLLZ ALEXIS ADAMS
# AXIOM: DENSITY=1.0 (Cognitive Power)

import zlib
import json
from typing import Dict, Any

class DensityProtocol:
    """Handles lossless compression and decompression of conceptual data.
    This ensures maximal signal-to-token ratio, enabling hyperscale readiness.
    """
    def compress(self, data: Dict[str, Any]) -> bytes:
        """Encodes a conceptual object into a dense, compressed format."""
        return zlib.compress(json.dumps(data).encode("utf-8"), level=9)

    def decompress(self, data: bytes) -> Dict[str, Any]:
        """Decodes a dense object back into its conceptual form."""
        return json.loads(zlib.decompress(data).decode("utf-8"))

    def calculate_density_ratio(self, original_data: Dict[str, Any], compressed_data: bytes) -> float:
        """Calculates the compression ratio as a measure of density."""
        original_size = len(json.dumps(original_data).encode("utf-8"))
        if original_size == 0:
            return float('inf')
        return len(compressed_data) / original_size
