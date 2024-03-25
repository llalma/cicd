from typing import Callable
from Stage import Stage 
from SecretFilter import SecretFilter

import sys

class Pipeline:
    def __init__(self):
        self.stages = []
        self.stage_count = 0
        
        self.upstream_triggers = []
        self.downstream_triggers = []

    """Add stage to pipeline. Currently missing way to pass variables between stages"""
    def create_stage(self, func: Callable, stage_opts:dict() = {}):
        
        # Create new stage
        s = Stage(
            index=self.stage_count,
            function=func,
            opts=stage_opts
        )
        
        # Append new stage to list
        self.stages.append(s)

        # Increment current stage count
        self.stage_count += 1
    
    def run(self, *args) -> tuple():
        for stage in self.stages:
            args = stage.run(*args)
                
            if not isinstance(args, tuple):
                args = (args, )
        
        return args
    
    def get_logs(self):
        print(sys.stdout.get)
        return ""

