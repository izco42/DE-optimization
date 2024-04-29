import matplotlib.pyplot as plt
import numpy as np

fitness_values = [
    2.7434458197735236,
    1.9418435265191811,
    0.26116880448089824,
    0.26116880448089824,
    0.17737631630292688,
    0.10299682864883965,
    0.014782572452234745,
    0.009251672212748254,
    0.0015582640194990902,
    0.0015582640194990902,
    0.0015582640194990902,
    0.00001586400955306464,
    0.00001586400955306464,
    0.00001586400955306464,
    0.000001246607340890888,
    0.000001246607340890888,
    0.000001246607340890888,
    0.0000002933904497841887,
    0.0000002933904497841887,
    0.000000026905702383755087
]

mean_fitness = np.mean(fitness_values)
std_dev_fitness = np.std(fitness_values)

print("Promedio de los valores de fitness:", mean_fitness)
print("Desviación estándar de los valores de fitness:", std_dev_fitness)

# Crear una lista de índices para el eje x
indices = list(range(1, len(fitness_values) + 1))

plt.figure(figsize=(10, 6))
plt.plot(indices, fitness_values, marker='o', linestyle='-')
plt.title('Funcion Sphere')
plt.xlabel('Iteración')
plt.ylabel('Valor de aptitud')
plt.grid(True)
plt.show()
