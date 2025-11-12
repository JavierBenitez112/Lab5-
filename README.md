<img width="783" height="632" alt="image" src="https://github.com/user-attachments/assets/d6c974ab-3809-436f-a189-74554fa71021" />
<img width="784" height="613" alt="image" src="https://github.com/user-attachments/assets/865d5a87-6787-470f-92ac-b2ad79e416aa" />
<img width="781" height="594" alt="image" src="https://github.com/user-attachments/assets/77eaf42b-0520-4a44-bc11-8c5f1ed37820" />


# Sistema Solar con Shaders Procedurales

Proyecto de gráficos por computadora que implementa un sistema solar completo usando únicamente shaders procedurales (sin texturas ni materiales externos). Todos los efectos visuales se generan mediante cálculos matemáticos en los fragment shaders y vertex shaders.

## Características Implementadas

### Planetas (5 totales)
1. **Planeta Rocoso** - Superficie rocosa con variaciones de terreno, gradientes de altitud, iluminación día/noche y efectos de erosión
2. **Gigante Gaseoso** - Bandas de gas animadas, ondas turbulentas, iluminación atmosférica y remolinos procedurales
3. **Planeta Sci-Fi** - Efectos de energía pulsante, redes de circuitos, gradientes dinámicos y brillo procedural
4. **Planeta Helado** (Adicional) - Superficie de hielo con fracturas, capas de nieve, reflexión de hielo y cristales
5. **Planeta Volcánico** (Adicional) - Superficie de lava, flujos animados, iluminación incandescente y efectos de humo

### Características Especiales
- **Anillos Procedurales** - Anillos planetarios generados proceduralmente alrededor del gigante gaseoso usando geometría de disco
- **Luna Procedural** - Luna que orbita alrededor del planeta rocoso con superficie cratérica generada proceduralmente
- **Sistema Orbital** - Todos los planetas orbitan alrededor del centro del sistema solar con rotación propia

## Arquitectura del Sistema

### Shaders (src/shaders.rs)

#### Vertex Shader
- **Función**: `vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex`
- **Parámetros**:
  - `vertex`: Vértice original con posición, normal, coordenadas de textura
  - `uniforms`: Matrices de transformación (model, view, projection, viewport) y tiempo
- **Transformaciones**:
  1. Aplica matriz de modelo (traslación, rotación, escala)
  2. Aplica matriz de vista (cámara)
  3. Aplica matriz de proyección (perspectiva)
  4. Convierte a coordenadas normalizadas (NDC)
  5. Aplica matriz de viewport (pantalla)

#### Fragment Shaders

Cada planeta tiene un fragment shader con **4 capas de cálculo**:

##### Planeta Rocoso (`shader_rocky_planet`)
- **CAPA 1**: Ruido fractal para terreno base (4 octaves)
- **CAPA 2**: Gradientes de altitud simulados basados en latitud
- **CAPA 3**: Iluminación simulada con terminador (día/noche) usando producto punto con dirección solar
- **CAPA 4**: Efectos de erosión y valles con ruido procedural

**Uniforms utilizados**:
- `time`: Tiempo transcurrido para animaciones
- `world_position`: Posición del fragmento en espacio mundial
- `color`: Color base del fragmento (iluminación del sistema)

##### Gigante Gaseoso (`shader_gas_giant`)
- **CAPA 1**: Bandas de latitud con gradientes suaves
- **CAPA 2**: Ondas de gas turbulentas animadas (3 ondas superpuestas)
- **CAPA 3**: Iluminación simulada con gradiente de profundidad atmosférica
- **CAPA 4**: Remolinos y vórtices procedurales usando funciones trigonométricas

**Uniforms utilizados**:
- `time`: Para animación de ondas y remolinos
- Coordenadas esféricas (`theta`, `phi`) para patrones

##### Planeta Sci-Fi (`shader_scifi_planet`)
- **CAPA 1**: Patrones de energía pulsante con múltiples frecuencias (3 pulsos)
- **CAPA 2**: Redes de circuitos y nodos energéticos con detección de patrones
- **CAPA 3**: Gradientes de color dinámicos con iluminación simulada direccional
- **CAPA 4**: Efectos de brillo y resplandor procedural

**Uniforms utilizados**:
- `time`: Para pulsaciones y animaciones
- Producto punto con dirección de energía para iluminación

##### Planeta Helado (`shader_ice_planet`)
- **CAPA 1**: Superficie de hielo con fracturas usando ruido fractal (5 octaves)
- **CAPA 2**: Capas de nieve con gradientes de profundidad
- **CAPA 3**: Iluminación simulada con reflexión de hielo (brillo aumentado)
- **CAPA 4**: Efectos de cristales y escarcha con patrones de detección

**Uniforms utilizados**:
- `time`: Para animación de fracturas
- Reflexión calculada con producto punto

##### Planeta Volcánico (`shader_volcanic_planet`)
- **CAPA 1**: Superficie de lava y roca fundida con ruido fractal
- **CAPA 2**: Flujos de lava animados (2 ondas superpuestas)
- **CAPA 3**: Iluminación simulada de lava incandescente (pulsante)
- **CAPA 4**: Efectos de humo y ceniza con ruido procedural

**Uniforms utilizados**:
- `time`: Para flujos de lava y pulsaciones de brillo

##### Anillos (`shader_rings`)
- Distancia radial desde el centro
- Bandas de anillos usando funciones sinusoidales
- Variación de densidad con ruido fractal
- Color gris con variaciones de densidad

**Uniforms utilizados**:
- `time`: Para animación de bandas
- `world_position`: Para calcular distancia radial

##### Luna (`shader_moon`)
- Cráteres con ruido fractal (5 octaves)
- Variaciones de superficie lunar
- Colores grises con variaciones de profundidad

**Uniforms utilizados**:
- `time`: Para variaciones sutiles

### Funciones Helper

#### `noise_3d(pos: Vector3, time: f32) -> f32`
Genera ruido procedural 3D usando funciones sinusoidales.
- **Parámetros**:
  - `pos`: Posición en espacio 3D
  - `time`: Tiempo para animación
- **Retorna**: Valor de ruido entre 0.0 y 1.0

#### `fractal_noise(pos: Vector3, time: f32, octaves: i32) -> f32`
Genera ruido fractal con múltiples octaves (ruido de Perlin simplificado).
- **Parámetros**:
  - `pos`: Posición en espacio 3D
  - `time`: Tiempo para animación
  - `octaves`: Número de capas de ruido (típicamente 3-5)
- **Retorna**: Valor de ruido fractal entre 0.0 y 1.0
- **Algoritmo**: Suma múltiples octaves con amplitud y frecuencia decrecientes (0.5, 0.25, 0.125...)

#### `spherical_coords(pos: Vector3) -> (f32, f32, f32)`
Convierte coordenadas cartesianas a esféricas.
- **Parámetros**: `pos`: Posición en espacio 3D
- **Retorna**: Tupla `(r, theta, phi)`
  - `r`: Radio (distancia desde origen)
  - `theta`: Latitud (ángulo desde el plano XY)
  - `phi`: Longitud (ángulo en el plano XZ)

### Generación Procedural de Geometría (src/obj.rs)

#### `generate_sphere(radius: f32, segments: u32) -> Obj`
Genera una esfera programáticamente.
- **Parámetros**:
  - `radius`: Radio de la esfera
  - `segments`: Número de segmentos en latitud y longitud (mayor = más suave)
- **Geometría**: Esfera completa con normales y coordenadas de textura
- **Uso**: Base para todos los planetas y la luna

#### `generate_rings(inner_radius: f32, outer_radius: f32, segments_radial: u32, segments_angular: u32) -> Obj`
Genera anillos planetarios usando un disco delgado.
- **Parámetros**:
  - `inner_radius`: Radio interno del anillo
  - `outer_radius`: Radio externo del anillo
  - `segments_radial`: Segmentos radiales (del centro al borde)
  - `segments_angular`: Segmentos angulares (alrededor del círculo)
- **Geometría**: Disco plano en el plano Y=0 con normales hacia arriba
- **Uso**: Anillos alrededor del gigante gaseoso

### Sistema de Órbitas (src/main.rs)

#### Estructura `Planet`
```rust
struct Planet {
    orbital_radius: f32,      // Distancia del centro
    orbital_angle: f32,        // Ángulo actual en la órbita
    orbital_speed: f32,        // Velocidad angular (rad/s)
    rotation_speed: f32,      // Velocidad de rotación propia
    scale: f32,                // Escala del planeta
    planet_type: PlanetType,   // Tipo de shader
    orbital_plane: Vector3,    // Plano de órbita
}
```

#### Uniforms (`Uniforms`)
```rust
pub struct Uniforms {
    pub model_matrix: Matrix,        // Transformación del modelo
    pub view_matrix: Matrix,          // Transformación de la cámara
    pub projection_matrix: Matrix,    // Proyección perspectiva
    pub viewport_matrix: Matrix,      // Transformación al viewport
    pub time: f32,                    // Tiempo transcurrido
}
```

#### Cálculo de Órbitas
- **Posición orbital**: `orbit_x = radius * cos(angle)`, `orbit_z = radius * sin(angle)`
- **Rotación propia**: `rotation_y = elapsed_time * rotation_speed`
- **Actualización**: `orbital_angle += orbital_speed * delta_time`

#### Luna
- **Órbita**: Alrededor del planeta rocoso (índice 0)
- **Radio orbital**: 1.5 unidades desde el planeta
- **Velocidad**: 1.0 rad/s
- **Rotación propia**: 0.1 rad/s

#### Anillos
- **Planeta asociado**: Gigante gaseoso (índice 1)
- **Radio interno**: 2.5 unidades
- **Radio externo**: 3.5 unidades
- **Posición**: Misma que el planeta, sin rotación propia

## Control de Cámara

- **W/S**: Rotar cámara arriba/abajo (pitch)
- **A/D**: Rotar cámara izquierda/derecha (yaw)
- **Flechas Arriba/Abajo**: Zoom in/out
- **Q/E o Flechas Izquierda/Derecha**: Pan horizontal
- **R/F**: Pan vertical

## Estructura del Proyecto

```
src/
├── main.rs          # Loop principal, sistema de órbitas, renderizado
├── shaders.rs       # Todos los shaders (vertex y fragment)
├── obj.rs           # Generación procedural de geometría
├── vertex.rs        # Estructura de vértices
├── fragment.rs      # Estructura de fragmentos
├── framebuffer.rs   # Manejo del framebuffer
├── triangle.rs      # Rasterización de triángulos
├── matrix.rs        # Operaciones de matrices
├── camera.rs        # Control de cámara
└── light.rs         # Sistema de iluminación
```

## Compilación y Ejecución

```bash
cargo build
cargo run
```

**Nota**: Requiere Clang/LLVM para compilar raylib-sys en Windows. Si hay problemas de compilación, asegúrate de tener las herramientas de desarrollo necesarias instaladas.

## Notas Técnicas

- Todos los efectos visuales son **100% procedurales** - no se usan texturas ni imágenes
- Los shaders usan **coordenadas esféricas** para crear patrones basados en latitud/longitud
- El **ruido fractal** se genera usando múltiples octaves de funciones sinusoidales
- La **iluminación simulada** se calcula usando productos punto con direcciones de luz
- Los **gradientes** se crean interpolando entre múltiples colores basados en valores calculados
- Las **animaciones** usan el uniform `time` que se actualiza cada frame
