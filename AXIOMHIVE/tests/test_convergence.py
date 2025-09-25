# @AXIOMHIVE @DEVDOLLZ ALEXIS ADAMS
# Test Suite for the Convergence Engine. AXIOM: FLAW=0.

import unittest
import os
import json
from typing import Dict, Any
from src.core.sovereignty_protocol import SovereigntyProtocol
from src.core.density_protocol import DensityProtocol
from src.agents.cerebrum import Cerebrum
from src.agents.hadrian import Hadrian

class TestConvergenceProtocol(unittest.TestCase):

    def setUp(self):
        self.test_log_file = "test_audit.log"
        if os.path.exists(self.test_log_file):
            os.remove(self.test_log_file)

    def tearDown(self):
        if os.path.exists(self.test_log_file):
            os.remove(self.test_log_file)

    def test_axiom_sovereignty(self):
        """Verify Zero Egress and Auditable Logging."""
        protocol = SovereigntyProtocol(self.test_log_file)
        with self.assertRaises(PermissionError):
            protocol.enforce_zero_egress(b"some_data")
        
        protocol.log_action("TestAgent", "TestAction", {}, "testhash")
        self.assertTrue(os.path.exists(self.test_log_file))

    def test_axiom_density(self):
        """Verify lossless compression."""
        protocol = DensityProtocol()
        original: Dict[str, Any] = {"key": "value", "data": [1, 2, 3], "long_text": "This is a longer text to ensure compression works properly and reduces size."}
        compressed = protocol.compress(original)
        decompressed = protocol.decompress(compressed)
        self.assertIsInstance(compressed, bytes)
        self.assertGreater(len(json.dumps(original).encode("utf-8")), len(compressed))
        self.assertEqual(original, decompressed)

    def test_cerebrum_diagnosis(self):
        """Verify strategic diagnosis from intent."""
        cerebrum = Cerebrum()
        directive = "Please can you design a system to create a data moat"
        clean = cerebrum.filter_noise(directive)
        strategy = cerebrum.diagnose_strategy(clean)
        self.assertEqual(clean, "system to create data moat")
        self.assertEqual(strategy["primary_moat"], "Data")
        self.assertEqual(strategy["complexity_type"], "System/Technical")

    def test_full_symbiotic_cycle(self):
        """Test the full workflow from directive to output."""
        cerebrum = Cerebrum()
        hadrian = Hadrian()
        
        directive = "reduce complexity in our architecture"
        clean_directive = cerebrum.filter_noise(directive)
        strategy = cerebrum.diagnose_strategy(clean_directive)
        output = hadrian.orchestrate_moat_construction(strategy)
        
        self.assertIn("Operational", output)
        self.assertIn("GENERATE_SIMPLIFICATION_PLAN", output)

if __name__ == "__main__":
    unittest.main()
