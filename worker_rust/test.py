import CICD_TBD


@CICD_TBD.stage_wrapper
def func():
    print("in func")


def func2():
    print("in func2")


with CICD_TBD.withfunc("stage 1") as stage:
    print(stage.name)

func()
