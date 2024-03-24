import pytest
from typing import Callable

from Stage import Stage

"""Test stage name can be set"""
@pytest.mark.parametrize("input, expected", 
                         [("gsdf", "gsdf")])
def test_stage_name(input: str, expected:str):
    stage = Stage(index=0, opts={'name':input}, function=lambda x: x)
    assert expected == stage.get_name()

"""Test that a stage can return values"""
@pytest.mark.parametrize("function, expected",
                         [(lambda: 4, 4)])
def test_return_stage(function: Callable, expected:str):
    stage = Stage(index=0, function=function, opts={})
    assert expected == stage.run()

"""Test that parameters can be passed to the stage and be used"""
@pytest.mark.parametrize("input, function, expected",
                         [([1,2,3,4], lambda input: [x*2 for x in input], [2,4,6,8])])
def test_parameter_stage(input:list(), function: Callable, expected:str):
    stage = Stage(index=0, function=function, opts={})
    assert expected == stage.run(input)
