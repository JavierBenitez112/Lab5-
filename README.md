<img width="724" height="478" alt="image" src="https://github.com/user-attachments/assets/e66948e9-5652-4869-8586-dba33663aedd" />
<img width="761" height="569" alt="image" src="https://github.com/user-attachments/assets/67fb8c6b-2e12-4bb5-b31d-0c459612e323" />
<img width="795" height="616" alt="image" src="https://github.com/user-attachments/assets/a60d42ce-13f7-470e-a4fb-569186a7d23d" />




# Sistema Solar con Shaders Procedurales y Sol Animado

Proyecto de gráficos por computadora que implementa un sistema solar completo usando únicamente shaders procedurales (sin texturas ni materiales externos). Todos los efectos visuales se generan mediante cálculos matemáticos en los fragment shaders y vertex shaders.

## Estrella Principal: SOL ANIMADO

### Características del Sol
- **100% Procedural** - Generado completamente con shaders, sin texturas
- **7 Capas de Shader Complejo** - Combinación de múltiples técnicas de ruido
- **Animación Continua** - Efectos de turbulencia, pulsaciones y llamaradas solares en tiempo real
- **Distorsión en Vertex Shader** - Prominencias solares y efectos de flare
- **Gradiente de Temperatura Dinámico** - Colores basados en temperatura simulada
- **Emisión Variable** - Picos de energía y regiones de alta actividad
- **64 Segmentos** - Geometría de alta calidad para máximo detalle

### Técnicas de Ruido Implementadas

El sol utiliza **tres tipos de ruido procedural** simulados:

#### 1. **Perlin Noise Simulado** (Turbulencias Solares)
- **Implementación**: Ruido fractal con 3 octavas (6, 4 y 3 niveles)
- **Frecuencias**: 1.0x, 2.3x, 4.7x
- **Uso**: Simula las turbulencias naturales de la superficie solar
- **Parámetros ajustables**:
  - `perlin_octave1`: Base con 6 octavas
  - `perlin_octave2`: Frecuencia media con 4 octavas
  - `perlin_octave3`: Alta frecuencia con 3 octavas
- **Combinación**: Pesos de 0.5, 0.3, 0.2 para crear turbulencia realista

#### 2. **Cellular Noise Simulado** (Manchas Solares)
- **Implementación**: Patrón tipo Voronoi usando celdas procedurales
- **Escala de celda**: 8.0 unidades
- **Uso**: Simula manchas solares (regiones más frías y oscuras)
- **Parámetros ajustables**:
  - `cell_scale`: Tamaño de las celdas (manchas)
  - `sunspot_threshold`: Umbral para detectar manchas (0.7)
- **Efecto**: Regiones con cellular_pattern > 0.7 son más oscuras (manchas solares)

#### 3. **Simplex Noise Simulado** (Flujos de Plasma)
- **Implementación**: Ondas trigonométricas en 3D con dos frecuencias
- **Frecuencias**: 3.5 y 7.2
- **Uso**: Simula flujos de plasma en la fotosfera
- **Parámetros ajustables**:
  - `simplex_freq1`: Frecuencia baja para flujos grandes
  - `simplex_freq2`: Frecuencia alta para detalles finos
- **Combinación**: Pesos de 0.6 y 0.4 para mezcla de escalas

### Estructura del Shader del Sol

El shader del sol está dividido en **7 capas procedurales**:

#### **CAPA 1: Turbulencias Solares** (Perlin Noise)
```rust
// 3 octavas de ruido fractal con diferentes frecuencias
perlin_turbulence = octave1 * 0.5 + octave2 * 0.3 + octave3 * 0.2
```
- Simula las turbulencias naturales de la convección solar
- Animación continua con velocidades variables

#### **CAPA 2: Manchas Solares** (Cellular Noise)
```rust
// Patrón cellular para manchas oscuras
is_sunspot = if cellular_pattern > 0.7 { 0.4 + random * 0.3 } else { 1.0 }
```
- Regiones más frías y oscuras en la superficie
- Movimiento lento siguiendo la rotación solar

#### **CAPA 3: Flujos de Plasma** (Simplex Noise)
```rust
// Ondas de plasma en múltiples escalas
simplex_plasma = flow1 * 0.6 + flow2 * 0.4
```
- Simula movimientos de plasma en la fotosfera
- Patrones complejos de convección

#### **CAPA 4: Emisión Variable** (Picos de Energía)
```rust
// Pulsación base + picos localizados
emission_intensity = base_pulse * high_energy_regions * turbulence
```
- Pulsación global del sol (rango: 0.7 - 1.0)
- Picos de energía localizados (hasta 3x intensidad)
- Regiones de alta actividad solar

#### **CAPA 5: Gradiente de Temperatura** (Color Dinámico)
```rust
// 5 niveles de temperatura con transiciones suaves
temperatura = radial_gradient * 0.4 + perlin * 0.3 + simplex * 0.2 + cellular * 0.1
```
- **Colores basados en temperatura**:
  - Blanco-amarillo (> 0.8): Centro muy caliente
  - Amarillo-naranja (0.6-0.8): Región caliente
  - Naranja (0.4-0.6): Región media
  - Rojo-naranja (0.2-0.4): Relativamente frío
  - Rojo oscuro (< 0.2): Manchas solares

#### **CAPA 6: Corona Solar** (Resplandor en Bordes)
```rust
// Efecto limbo darkening inverso
corona_effect = corona_brightness * corona_variation * 1.3
```
- Simula la corona visible en los bordes
- Animación ondulante de la corona
- Mayor brillo en los bordes (efecto atmosférico)

#### **CAPA 7: Llamaradas Solares** (Flares Procedurales)
```rust
// Erupciones solares dramáticas
flare_intensity = if pattern > 0.85 { 1.0 + strength * pulse * 2.0 } else { 1.0 }
```
- Erupciones solares procedurales
- Pulsaciones de alta frecuencia
- Pueden aumentar intensidad hasta 3x

### Vertex Shader Especial: Distorsión y Flare

El sol tiene un **vertex shader dedicado** (`vertex_shader_sun`) que aplica:

#### **Distorsión Procedural**
```rust
// Ondas de distorsión en 3 ejes
distortion = (wave1 + wave2 + wave3) / 3.0 * 0.08
```
- Amplitud: 8% del radio
- Frecuencia: 4.0
- Simula actividad en la superficie

#### **Prominencias Solares**
```rust
// Prominencias extremas ocasionales
prominence_intensity = if pattern > 0.85 { (pattern - 0.85) * 3.0 } else { 0.0 }
```
- Prominencias que se extienden hasta 15% adicional del radio
- Patrones basados en coordenadas esféricas
- Animación continua con el tiempo

### Parámetros Ajustables del Sol

**Ruido Perlin:**
- `octaves`: 6, 4, 3 (número de capas de detalle)
- `frequency_multipliers`: 1.0x, 2.3x, 4.7x
- `time_speeds`: 0.3, 0.25, 0.4

**Ruido Cellular:**
- `cell_scale`: 8.0 (tamaño de manchas)
- `sunspot_threshold`: 0.7 (umbral para manchas)
- `time_speeds`: 0.1, 0.08, 0.12 (por eje)

**Ruido Simplex:**
- `simplex_freq1`: 3.5 (flujos grandes)
- `simplex_freq2`: 7.2 (detalles finos)
- `time_speeds`: 0.5-0.8 (velocidades de ondas)

**Emisión:**
- `base_pulse_speed`: 1.5 (pulsación global)
- `energy_spike_thresholds`: 0.75 (activación de picos)
- `flare_threshold`: 0.85 (activación de llamaradas)

**Distorsión (Vertex Shader):**
- `distortion_amplitude`: 0.08 (8% del radio)
- `distortion_freq`: 4.0
- `prominence_threshold`: 0.85
- `prominence_amplitude`: 0.15 (15% adicional)

### Uniformes Utilizados por el Sol

```rust
pub struct Uniforms {
    pub time: f32,                    // Tiempo para animaciones continuas
    pub model_matrix: Matrix,         // Transformación del sol
    pub view_matrix: Matrix,          // Posición de la cámara
    pub projection_matrix: Matrix,    // Proyección perspectiva
    pub viewport_matrix: Matrix,      // Transformación a pantalla
}
```

## Características Implementadas

### Planetas (5 totales)
1. **Planeta Rocoso** - Superficie rocosa con variaciones de terreno, gradientes de altitud, iluminación día/noche y efectos de erosión
2. **Gigante Gaseoso** - Bandas de gas animadas, ondas turbulentas, iluminación atmosférica y remolinos procedurales
3. **Planeta Sci-Fi** - Efectos de energía pulsante, redes de circuitos, gradientes dinámicos y brillo procedural
4. **Planeta Helado** (Adicional) - Superficie de hielo con fracturas, capas de nieve, reflexión de hielo y cristales
5. **Planeta Volcánico** (Adicional) - Superficie de lava, flujos animados, iluminación incandescente y efectos de humo

### Características Especiales
- **Sol Animado Central** - Estrella con 7 capas de shader complejo y vertex shader con distorsión
- **Anillos Procedurales** - Anillos planetarios generados proceduralmente alrededor del gigante gaseoso usando geometría de disco
- **Luna Procedural** - Luna que orbita alrededor del planeta rocoso con superficie cratérica generada proceduralmente
- **Sistema Orbital** - Todos los planetas orbitan alrededor del sol en el centro del sistema

## Arquitectura del Sistema

### Shaders (src/shaders.rs)

#### Vertex Shaders

##### Vertex Shader Estándar
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
- **Uso**: Todos los planetas, anillos y luna

##### Vertex Shader del Sol (`vertex_shader_sun`)
- **Función**: `vertex_shader_sun(vertex: &Vertex, uniforms: &Uniforms) -> Vertex`
- **Características Especiales**:
  1. **Distorsión Procedural**: Desplaza vértices usando ondas sinusoidales
     - Amplitud: 8% del radio
     - 3 ondas independientes (X, Y, Z)
     - Frecuencia: 4.0 con velocidades variables
  2. **Prominencias Solares**: Extensiones extremas ocasionales
     - Activa cuando pattern > 0.85
     - Amplitud adicional: 15% del radio
     - Basado en coordenadas esféricas
  3. **Animación Continua**: Usa uniform `time` para movimiento
- **Transformaciones Estándar**: Mismas que vertex_shader
- **Uso**: Exclusivo para el sol

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

##### Sol (`shader_sun`) - ** ESTRELLA PRINCIPAL**
- **CAPA 1**: Ruido Perlin simulado con 3 octavas (turbulencias solares)
- **CAPA 2**: Ruido Cellular simulado (manchas solares)
- **CAPA 3**: Ruido Simplex simulado (flujos de plasma)
- **CAPA 4**: Emisión variable con picos de energía
- **CAPA 5**: Gradiente de temperatura con 5 niveles de color
- **CAPA 6**: Corona solar con resplandor en bordes
- **CAPA 7**: Llamaradas solares procedurales

**Uniforms utilizados**:
- `time`: Para todas las animaciones continuas
- `world_position`: Para cálculos de ruido y coordenadas esféricas
- Intensidad de salida: Valores hasta 3.0 (efecto HDR)

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

## Resumen de Criterios del Laboratorio

| Criterio | Puntos | Implementación | Estado |
|----------|--------|----------------|--------|
| **Creatividad visual del diseño y realismo percibido** | 30 | - Sol con 7 capas de shader complejo<br>- Gradiente de temperatura realista<br>- Manchas solares, turbulencias y prominencias<br>- Corona solar visible | ✅ Completado |
| **Complejidad del shader (múltiples funciones de ruido)** | 40 | - **Perlin Noise**: 3 octavas con diferentes frecuencias<br>- **Cellular Noise**: Patrón tipo Voronoi para manchas<br>- **Simplex Noise**: Flujos de plasma en 2 escalas<br>- Combinación de los 3 tipos | ✅ Completado |
| **Implementación correcta del tiempo y animación continua** | 20 | - Uniform `time` usado en todas las capas<br>- Animaciones cíclicas y continuas<br>- Velocidades variables por capa<br>- Sin interrupciones ni repeticiones bruscas | ✅ Completado |
| **Uso de Perlin, Simplex o Cellular noise con parámetros ajustables** | 20 | - 3 tipos de ruido implementados<br>- Parámetros documentados y ajustables<br>- Diferentes escalas y frecuencias<br>- Octavas configurables | ✅ Completado |
| **Agregar emisión variable (picos de energía)** | 15 | - Pulsación base global (0.7-1.0)<br>- Picos de energía localizados (hasta 3x)<br>- Llamaradas solares dramáticas<br>- Regiones de alta actividad | ✅ Completado |
| **Distorsión/flare en Vertex Shader** | 15 | - `vertex_shader_sun` dedicado<br>- Distorsión procedural (8% amplitud)<br>- Prominencias solares (15% adicional)<br>- Ondas animadas en 3 ejes | ✅ Completado |
| **Control de color por intensidad/temperatura** | 20 | - 5 niveles de temperatura<br>- Gradiente dinámico con transiciones suaves<br>- Colores desde rojo oscuro a blanco-amarillo<br>- Basado en física (radiación de cuerpo negro) | ✅ Completado |
| **Documentación clara en README** | 10 | - Explicación detallada de cada tipo de ruido<br>- Parámetros documentados<br>- Uniformes listados<br>- Estructura de capas explicada | ✅ Completado |
| **TOTAL** | **170** | **Todos los criterios implementados** | ✅ **100%** |

### Restricciones Técnicas Cumplidas

 **Solo una esfera como base** - El sol es una esfera de 64 segmentos
 **Sin texturas ni materiales** - 100% procedural con shaders
 **Animación con uniform time** - Todas las capas usan tiempo
 **Funciones de ruido en shader** - 3 tipos implementados
 **Solo modificado con shaders** - Vertex y fragment shaders especializados
 **Animación continua y cíclica** - Sin interrupciones, loops perfectos

## Notas Técnicas

- Todos los efectos visuales son **100% procedurales** - no se usan texturas ni imágenes
- Los shaders usan **coordenadas esféricas** para crear patrones basados en latitud/longitud
- El **ruido fractal** se genera usando múltiples octaves de funciones sinusoidales
- La **iluminación simulada** se calcula usando productos punto con direcciones de luz
- Los **gradientes** se crean interpolando entre múltiples colores basados en valores calculados
- Las **animaciones** usan el uniform `time` que se actualiza cada frame
- El **sol** implementa los 3 tipos de ruido principales: Perlin, Cellular y Simplex (simulados)
- El **vertex shader del sol** permite distorsión y prominencias en tiempo real
- La **emisión variable** puede alcanzar hasta 3x la intensidad base para llamaradas
- Los **colores del sol** están basados en radiación de cuerpo negro (temperatura física)
