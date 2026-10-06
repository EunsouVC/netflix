# Lume Netflix — experimental

Cliente independente para Windows 10/11 x64: **Tauri 2 + Microsoft Edge WebView2 Runtime**, abrindo `https://www.netflix.com/` em janela própria. Não é afiliado à Netflix. Nome e ícone são próprios; Netflix é marca de seus titulares.

**DRM e reprodução não são garantidos.** Fazer login ou navegar pelo catálogo não comprova que vídeos funcionam. WebView2/Tauri não aparece na lista de navegadores oficialmente suportados pela Netflix. Pode haver navegador não suportado, tela preta, falha de licença, erro de reprodução ou resolução limitada. Não há promessa de 1080p, 4K, HDR, áudio específico ou downloads offline. O projeto não instala CDM, não contorna DRM e não altera o user-agent.

## O que vem pronto

- Uma janela de 1100 × 720 unidades lógicas, redimensionável, centralizada e limitada à área disponível ao abrir. Mínimo de 640 × 420.
- Perfil persistente em `%LOCALAPPDATA%\io.lume.netflix.experimental\WebView2`; cookies e armazenamento pertencem a este app. A Netflix ainda pode expirar ou revogar uma sessão.
- GPU/aceleração de hardware conforme as decisões padrão do WebView2, hardware e drivers disponíveis. Não força GPU nem desativa sandbox.
- Instância única: a segunda abertura restaura/foca a primeira. Ao fechar a última janela, o aplicativo encerra normalmente.
- Menu nativo: Início (`Ctrl+Shift+H`), Voltar (`Alt+←`), Recarregar (`F5`), Tela cheia (`F11`), Sobre e Sair.
- Ícone original, identificação própria na barra de tarefas e atalho no Menu Iniciar criado pelo instalador.
- Sem Electron, Node no aplicativo, frontend com framework, atualizador próprio, bandeja, telemetria própria ou tarefas periódicas.

O Runtime é compartilhado e separado do **navegador Edge**: não é necessário instalar/abrir o navegador. Processos `msedgewebview2.exe` são normais; o WebView2 usa processos auxiliares para renderização, GPU e rede. O app não promete um número fixo de processos nem uma quantidade de RAM. Um catálogo pesado e vídeo podem consumir bastante memória. Não há comparação de consumo medida nesta entrega.

## Build curto no Windows

Instale uma vez:

1. [Rust via rustup](https://www.rust-lang.org/tools/install), usando o toolchain MSVC estável.
2. [Visual Studio Build Tools](https://visualstudio.microsoft.com/downloads/), com **Desenvolvimento para desktop com C++**, MSVC e Windows SDK.
3. [WebView2 Runtime Evergreen](https://developer.microsoft.com/microsoft-edge/webview2/) para executar/testar. Isso é o Runtime, não o navegador Edge.

Na pasta que contém este README, em PowerShell:

```powershell
cargo install tauri-cli --version "^2" --locked
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-windows.ps1
```

O padrão gera um instalador **NSIS `.exe` por usuário**. Para EXE e MSI:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\build-windows.ps1 -Bundle all
```

Os instaladores, hashes SHA-256 e `Cargo.lock` gerado ficam em `dist\`. A primeira compilação baixa dependências Rust e ferramentas de empacotamento. WiX/MSI pode exigir o recurso opcional do Windows **.NET Framework 3.5**; se necessário, habilite-o em “Ativar ou desativar recursos do Windows” ou gere apenas NSIS. Node/npm não são necessários.

Para desenvolvimento e verificações:

```powershell
cargo tauri dev
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml
```

O Tauri está fixado em `2.12.1`. CLI, plugin e `tauri-build` usam a série 2. **Não há Cargo.lock nesta entrega**, porque não foi possível resolver/compilar dependências aqui. A primeira compilação cria o lock; guarde/adicione `src-tauri/Cargo.lock` ao repositório e use `--locked` nas recompilações para fixar também as dependências transitivas. `rust-toolchain.toml` usa o canal estável; guarde a versão concreta do Rust junto ao lock para reproduzir um build publicado.

## Alternativa sem instalar compilador no seu PC

Coloque **o conteúdo desta pasta** na raiz de um repositório GitHub seu, incluindo `.github/`. Em **Actions → Gerar instaladores Windows → Run workflow**, execute a rotina manual. Quando concluir, baixe o artefato `lume-netflix-windows-x64`: ele contém EXE, MSI, lock e hashes. O workflow faz `cargo check`, testa a política de navegação e compila os instaladores no Windows. Não publica releases automaticamente. Esse workflow foi preparado, mas não foi executado nesta entrega.

Esse é o caminho para não instalar Rust, Visual Studio, Node ou qualquer ferramenta de desenvolvimento no seu computador. Se você receber um `Lume-Netflix-*-setup.exe` já compilado, basta executá-lo. O instalador é por usuário e não exige permissões de administrador; ele pode instalar somente o **WebView2 Runtime Evergreen** caso o Runtime ainda não exista. Esse Runtime é um componente compartilhado da Microsoft necessário para renderizar a página, não o navegador Edge e não uma ferramenta de desenvolvimento. Sem ele, nenhum cliente Tauri/WebView2 consegue abrir a Netflix.

O `winget` não compila este projeto e não transforma o ZIP em um EXE. Ele pode instalar o Runtime separadamente, mas isso adicionaria uma etapa e não substitui a compilação. Por isso não há um comando `winget install` no pacote.

## Instalar e abrir

1. Execute o `*-setup.exe` produzido, ou o `.msi`. Escolha somente um formato.
2. Se faltar o WebView2, o instalador baixa o bootstrapper oficial da Microsoft e instala o Runtime Evergreen. Esse passo requer internet; o streaming também.
3. Abra **Lume Netflix** pelo Menu Iniciar. Se desejar, fixe-o na barra de tarefas.
4. Faça login diretamente na página da Netflix e escolha um perfil.

Os builds não têm assinatura digital configurada; o Windows pode informar editor desconhecido/SmartScreen. O README e o fonte permitem revisão antes de compilar. A desinstalação é feita pelas configurações de aplicativos do Windows. O perfil pode permanecer para preservar a sessão; para removê-lo, feche o app, saia da conta e apague **somente** a pasta de dados deste aplicativo. Não remova o Runtime compartilhado para desinstalar o Lume.

## Teste curto de login, reprodução e DRM

1. Feche o navegador e abra o app. Confirme janela própria, ícone e atalho. Abra novamente: deve continuar com uma janela. Teste F11 e redimensionamento.
2. Entre na conta, feche o app normalmente e reabra. Confirme se o login e perfil persistem. Não compartilhe a pasta WebView2: ela contém dados de sessão.
3. Inicie um filme/episódio com sua assinatura válida, aguarde carregar e reproduza por 2–5 minutos. Teste áudio, legendas, avanço e tela cheia. Repita com outro título.
4. Se aparecer erro, registre a mensagem/código, título, Windows, versão do Runtime, GPU/driver e monitor. Navegação funcional não deve ser registrada como sucesso de DRM. Uma captura de tela preta também não comprova falha: conteúdo protegido pode impedir captura; verifique a imagem no monitor.
5. Se falhar, atualize Runtime e driver e repita. Compare o mesmo título em um navegador oficialmente suportado. Se funcionar nele e falhar aqui, trate como incompatibilidade deste cliente; não há correção garantida. Não compre codecs ou troque hardware apenas com base neste experimento.
6. No Gerenciador de Tarefas, observe o app e seus processos WebView2. Meça após 30 segundos parado e durante reprodução estável. Feche a janela e confira se os processos associados encerram após alguns segundos. O serviço de atualização do Runtime é separado do app.

Opcional: enquanto o app estiver aberto, `powershell -NoProfile -File .\scripts\measure-memory.ps1` mostra uma fotografia da árvore de processos, memória privada comprometida e soma de working sets. A soma de working sets conta páginas compartilhadas mais de uma vez; não representa RAM física exclusiva. Memória privada comprometida também não é a mesma coisa que RAM residente.

## Limites e privacidade

Somente navegações HTTPS para `netflix.com` e seus subdomínios são aceitas. Isso limita a janela ao serviço; recursos de CDN usados pela página continuam disponíveis. Popups e downloads são bloqueados para evitar janelas adicionais e arquivos locais. Alguns links de ajuda, pagamento ou autenticação que abram nova janela/domínio podem não funcionar; nesses casos, use o site em um navegador suportado. Não existe encaminhamento automático ao navegador.

Não há comandos Rust expostos à página, capabilities remotas, plugin de shell/arquivos ou acesso global à API Tauri. Os comandos do menu executam somente expressões fixas de voltar/recarregar. O projeto não lê senhas, não exporta cookies e não modifica o conteúdo da Netflix; a página e o Runtime mantêm seus próprios mecanismos de armazenamento e rede. A aceleração de hardware continua sujeita a drivers, políticas do sistema e configuração externa do Runtime.

## Estado desta entrega

Fonte, configurações, ícones, script de build e workflow incluídos. Inspeção estática e integridade do ZIP verificadas. **Não compilado, não instalado e sem teste de login/reprodução/DRM** neste ambiente, que não dispõe de Rust/MSVC/Windows SDK. Leia `VALIDACAO.md` para os detalhes. Nenhum EXE/MSI pré-compilado está incluído.

## Referências oficiais

- [Distribuição do WebView2](https://learn.microsoft.com/microsoft-edge/webview2/concepts/distribution): Runtime Evergreen compartilhado e separado do navegador.
- [Instaladores Windows do Tauri 2](https://v2.tauri.app/distribute/windows-installer/): NSIS/MSI e bootstrapper do Runtime.
- [APIs de janela WebView do Tauri](https://docs.rs/tauri/2.12.1/tauri/webview/struct.WebviewWindowBuilder.html): perfil, navegação e janela.
- [Netflix no Windows](https://help.netflix.com/pt/node/23931) e [navegadores suportados](https://help.netflix.com/pt/node/30081): suporte oficial e requisitos de reprodução.

