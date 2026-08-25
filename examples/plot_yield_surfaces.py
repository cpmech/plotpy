import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from mpl_toolkits.mplot3d.art3d import Poly3DCollection

def plot_von_mises(elev, azim):
    fig = plt.figure(figsize=(8, 8))
    ax = fig.add_subplot(111, projection='3d')
    
    # Cylinder parameters
    radius = 3
    height = 15
    
    # Generate cylinder grid
    z_cyl = np.linspace(0, height, 50)
    theta = np.linspace(0, 2*np.pi, 50)
    theta_grid, z_grid = np.meshgrid(theta, z_cyl)
    
    x_cyl = radius * np.cos(theta_grid)
    y_cyl = radius * np.sin(theta_grid)
    
    # Rotation matrix to align cylinder with space diagonal (sigma_1 = sigma_2 = sigma_3)
    # The space diagonal direction is [1, 1, 1]
    # We rotate from [0, 0, 1] (Z-axis) to [1/sqrt(3), 1/sqrt(3), 1/sqrt(3)]
    
    v = np.array([1, 1, 1]) / np.sqrt(3)
    z_axis = np.array([0, 0, 1])
    
    # Cross product and angle for rotation
    axis = np.cross(z_axis, v)
    axis = axis / np.linalg.norm(axis)
    angle = np.arccos(np.dot(z_axis, v))
    
    # Rodrigues' rotation formula components
    K = np.array([[0, -axis[2], axis[1]],
                  [axis[2], 0, -axis[0]],
                  [-axis[1], axis[0], 0]])
    I = np.eye(3)
    R = I + np.sin(angle)*K + (1 - np.cos(angle))*np.dot(K, K)
    
    # Apply rotation
    points = np.stack([x_cyl.flatten(), y_cyl.flatten(), z_grid.flatten()])
    rotated_points = R.dot(points)
    
    X = rotated_points[0, :].reshape(x_cyl.shape)
    Y = rotated_points[1, :].reshape(y_cyl.shape)
    Z = rotated_points[2, :].reshape(z_grid.shape) # <--- THIS LINE IS FIXED
    
    ax.plot_surface(X, Y, Z, alpha=0.6, color='blue', edgecolor='none')
    
    # Plot hydrostatic axis
    ax.plot([0, 15], [0, 15], [0, 15], color='red', linestyle='dashed', linewidth=2, label='Hydrostatic Axis')
    
    ax.set_xlabel(r'$\sigma_1$')
    ax.set_ylabel(r'$\sigma_2$')
    ax.set_zlabel(r'$\sigma_3$')
    ax.set_title('von Mises Yield Surface (Cylinder)')
    ax.set_xlim([0, 15])
    ax.set_ylim([0, 15])
    ax.set_zlim([0, 15])
    ax.view_init(elev, azim)
    
    plt.savefig('von_mises_surface.png',bbox_inches='tight', dpi=300)
    print("Saved von_mises_surface.png")

def plot_mohr_coulomb(elev, azim):
    # Simplified visual representation of the Mohr-Coulomb hexagonal pyramid
    fig = plt.figure(figsize=(8, 8))
    ax = fig.add_subplot(111, projection='3d')
    
    # Apex of the pyramid (tensile region, assumed at -2 for visual purposes)
    apex = np.array([-2, -2, -2])
    
    # Vertices of the irregular hexagon in the octahedral plane at a given p'
    # For a frictional material, compressive strength > tensile strength
    r_c = 6.0 # Compression radius
    r_e = 3.5 # Extension radius
    
    # Angles for the vertices (Lode angles corresponding to triaxial compression/extension)
    angles = np.array([0, 60, 120, 180, 240, 300]) * np.pi / 180.0
    radii = np.array([r_c, r_e, r_c, r_e, r_c, r_e])
    
    base_center = np.array([12, 12, 12])
    
    # Base vectors for the octahedral plane
    n = np.array([1, 1, 1]) / np.sqrt(3)
    u = np.array([1, -1, 0]) / np.sqrt(2)
    v_vec = np.cross(n, u)
    
    # Calculate base vertices
    base_vertices = []
    for ang, r in zip(angles, radii):
        point = base_center + r * (np.cos(ang) * u + np.sin(ang) * v_vec)
        base_vertices.append(point)
        
    base_vertices = np.array(base_vertices)
    
    # Plot faces
    faces = []
    for i in range(6):
        faces.append([apex, base_vertices[i], base_vertices[(i+1)%6]])
        
    poly3d = Poly3DCollection(faces, alpha=0.5, facecolors='green', edgecolors='black', linewidths=1)
    ax.add_collection3d(poly3d)
    
    # Plot hydrostatic axis
    ax.plot([-3, 15], [-3, 15], [-3, 15], color='red', linestyle='dashed', linewidth=2, label='Hydrostatic Axis')
    
    ax.set_xlabel(r'$\sigma_1$')
    ax.set_ylabel(r'$\sigma_2$')
    ax.set_zlabel(r'$\sigma_3$')
    ax.set_title('Mohr-Coulomb Yield Surface (Hexagonal Pyramid)')
    
    ax.set_xlim([-3, 15])
    ax.set_ylim([-3, 15])
    ax.set_zlim([-3, 15])
    ax.view_init(elev, azim)
    
    plt.savefig('mohr_coulomb_surface.png',bbox_inches='tight', dpi=300)
    print("Saved mohr_coulomb_surface.png")

if __name__ == '__main__':
    elev = 20
    azim = 20
    plot_von_mises(elev, azim)
    plot_mohr_coulomb(elev, azim)
