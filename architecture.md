graph TD
A[Source Code] --> B[Parser]
B --> C[Abstract Syntax Tree AST]
C --> D[Type Checker]
D --> E[Intermediate Representation IR]
E --> F{Execution Mode}
F -->|Interpreted| G[Bytecode Generator]
G --> H[Bytecode Interpreter]
F -->|Compiled| I[Code Generator]
I --> J[Native Machine Code]
H --> K[Execution]
J --> K
