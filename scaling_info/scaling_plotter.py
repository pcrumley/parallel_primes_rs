import matplotlib.pyplot as plt
import numpy as np

# for data see scaling_tests.txt
x = np.arange(1, 5)
y = np.array([4.088, 2.040, 1.387, 1.083])

# make the figure
plt.figure(figsize=(6,4))
plt.plot(x, (y/y[0])**-1, 'd', label='Measured Speed-up')
plt.plot(x, x, 'k-', lw=.2, label='Ideal Scaling')
plt.title('Strong Scaling Test')
plt.xlabel('number of threads')
plt.ylabel('Speed-up vs one thread')
plt.legend()
plt.savefig('strong_scaling.png')
