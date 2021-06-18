import os

os.remove("tasks.txt")
task_file = open("tasks.txt", "x")

import random

num_tasks = random.randint(1, 2 ** 4)

tasks = []
for i in range(num_tasks):
    max_cpu_time = random.randint(1, 2 ** 20)
    tasks.append(
        (
            max_cpu_time,
            random.randint(1, min(2 ** 16, max_cpu_time)),
            random.randint(0, 2 ** 11),
            random.randint(1, 2 ** 5)
        )
    )

for cpu_time, cpu_burst_len, io_burst_len, weight in tasks:
    task_file.write(f'{cpu_time} {cpu_burst_len} {io_burst_len} {weight}\n')
