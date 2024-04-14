
import pytest
from typing import Callable

from Pipeline import Pipeline

@pytest.mark.parametrize("input, expected", 
                         [(3, 3)])
def test_stage_count(input: str, expected: int):
    func = lambda: 4
    pipeline = Pipeline()
    
    for _ in range(input):
        pipeline.create_stage(func)

    assert expected == pipeline.stage_count

@pytest.mark.parametrize("input, expected", 
                         [(["a", "b", "c"], ["a", "b", "c"])])
def test_stage_name(input: [str], expected: [str]):
    func = lambda: 4
    pipeline = Pipeline()
    
    for name in input:
        pipeline.create_stage(func, stage_opts={'name':name})

    assert expected == [s.get_name() for s in pipeline.stages]

@pytest.mark.parametrize("input, loop_count, expected", 
                         [(2, 4, 65536)])
def test_return_pipeline(input: int, loop_count: int, expected: int):
    func = lambda x: x*x
    pipeline = Pipeline()
    
    for name in range(loop_count):
        pipeline.create_stage(func)
    
    pipeline.run(input)
    assert (expected,) == pipeline.run(input)


