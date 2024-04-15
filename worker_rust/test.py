import CICD_TBD


# @CICD_TBD.stage_wrapper
# def func():
#     print("in func")
#


with CICD_TBD.Pipeline("pipe") as pipeline:
    with CICD_TBD.Stage("stage 1") as stage:
        print(stage.name)

    with CICD_TBD.Stage("stage 2") as stage:
        print(stage.name)
