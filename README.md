# smart-terminal

Esqueleto de una **terminal inteligente** escrita en Rust.

En esta fase solo se define la estructura del proyecto: módulos, tipos y
responsabilidades. La lógica de cada componente se implementará más adelante.

## Estructura

```text
smart-terminal/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs              # Punto de entrada del binario
    ├── lib.rs               # Raíz de la biblioteca
    ├── app.rs               # Orquestación de la aplicación
    ├── cli.rs               # Argumentos de línea de comandos
    ├── config.rs            # Configuración
    ├── error.rs             # Errores comunes
    ├── terminal/
    │   ├── mod.rs
    │   ├── input.rs         # Lectura de entrada interactiva
    │   ├── output.rs        # Renderizado de salida
    │   └── session.rs       # Estado de la sesión
    ├── shell/
    │   ├── mod.rs
    │   ├── command.rs       # Modelo de comandos
    │   ├── parser.rs        # Parseo de la línea de comandos
    │   ├── executor.rs      # Ejecución de comandos
    │   └── history.rs       # Historial
    ├── ai/
    │   ├── mod.rs
    │   ├── client.rs        # Cliente del proveedor de IA
    │   ├── intent.rs        # Detección de intenciones
    │   ├── planner.rs       # Planificación de acciones
    │   ├── safety.rs        # Validación de seguridad
    │   └── prompt.rs        # Prompts del sistema
    ├── commands/
    │   ├── mod.rs
    │   ├── builtin.rs       # Comandos internos
    │   └── external.rs      # Comandos externos
    ├── plugins/
    │   ├── mod.rs
    │   └── registry.rs      # Registro de plugins
    └── ui/
        ├── mod.rs
        ├── prompt.rs        # Prompt visual
        └── theme.rs         # Tema y colores
```

## Enfoque

- La entrada del usuario se interpreta como lenguaje natural o como comando.
- El motor de IA transforma intenciones en planes y comandos.
- Todo comando generado pasa por una capa de seguridad antes de ejecutarse.
- La UI y el ejecutor están desacoplados para poder probarlos por separado.
