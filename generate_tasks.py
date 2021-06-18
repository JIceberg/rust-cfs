import os

os.remove("tasks.txt")
task_file = open("tasks.txt", "x")

import random

num_tasks = random.randint(0, 2 ** 4)

tasks = []
for i in range(num_tasks):
    tasks.append(
        (
            random.randint(0, 2 ** 20),
            random.randint(0, 2 ** 16),
            random.randint(0, 2 ** 16),
            random.randint(0, 2 ** 16)
        )
    )

for cpu_time, cpu_burst_len, io_burst_len, weight in tasks:
    task_file.write(f'{cpu_time} {cpu_burst_len} {io_burst_len} {weight}\n')
