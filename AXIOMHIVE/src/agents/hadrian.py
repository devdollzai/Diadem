# @AXIOMHIVE @DEVDOLLZ ALEXIS ADAMS
# Agent: Hadrian (Moat Construction & Orchestration)

from .dagger import Dagger
from typing import Dict, Any

class Hadrian:
    """Orchestrates Dagger agents to construct competitive moats."""

    def __init__(self):
        self._dagger = Dagger()

    def orchestrate_moat_construction(self, strategy: Dict[str, Any]) -> str:
        """Routes the strategic plan to the correct Dagger execution function."""
        core_task = strategy.get("core_task")
        
        print(f"HADRIAN :: Orchestrating Dagger agents for task: {core_task}.")

        if core_task == "GENERATE_DATA_LOOP_ARCHITECTURE":
            return self._dagger.reinforce_data_moat(strategy)
        elif core_task == "GENERATE_SIMPLIFICATION_WORKFLOW":
            return self._dagger.reduce_complexity_tax(strategy)
        else:
            return self._dagger.default_execution("Unrecognized strategic task.")
