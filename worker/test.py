from Pipeline import Pipeline

p = Pipeline()

def s_1(val):
    print(val)
    return val+'1'


p.create_stage(s_1)
p.create_stage(s_1)
p.create_stage(s_1)

p.run("str")

