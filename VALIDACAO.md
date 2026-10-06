# Validação da entrega

Data de preparação: 5 de outubro de 2026.

Executado neste ambiente:

- Leitura das APIs e opções na documentação oficial do Tauri, Microsoft e Netflix.
- Leitura/parse dos arquivos JSON e TOML, verificando caminhos de frontend e ícones, tipo do instalador e configuração do Runtime.
- Parse de sintaxe dos scripts PowerShell, sem executá-los para compilar.
- Verificação dos PNGs/ICO: RGBA, tamanhos e tamanhos múltiplos no ICO.
- Verificação de presença de fonte, build script, workflow, guia e licença no ZIP; teste de integridade e hash SHA-256.

Não executado:

- Resolução das dependências Cargo, geração de Cargo.lock e compilação Rust.
- `cargo check`, teste Rust de política de navegação, build/execução da rotina GitHub Actions.
- Geração/execução de EXE/MSI, instalação em máquina sem Edge, login, persistência real dos cookies, reprodução/DRM e medição de memória/processos.

Motivo: Rust, Cargo, Visual Studio Build Tools e Windows SDK não estão disponíveis neste ambiente. Esta entrega contém um projeto experimental de código-fonte, não um binário certificado. O teste real exige ferramentas de build e uma conta/assinatura Netflix do usuário; nenhuma credencial foi solicitada ou utilizada.

O workflow incluído executa as verificações de compilação e gera ambos os formatos em um runner Windows. O guia inclui o teste manual de reprodução, que precisa ocorrer depois do build. A lista acima não equivale a confirmação de compatibilidade DRM.

