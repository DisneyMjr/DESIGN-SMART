# Design Smart

**Design Smart** é um editor de diagramação e criação de documentos para impressão e tela. Permite organizar páginas, imagens, textos e elementos gráficos em projetos como revistas, catálogos, livros, folders e materiais de divulgação.

## Principais recursos

- Documentos com páginas e páginas espelhadas (spreads).
- Quadros de texto e texto encadeado entre páginas.
- Estilos de parágrafo e caracteres, cores e amostras.
- Posicionamento de imagens, formas, alinhamento e organização dos elementos.
- Importação e exportação IDML conforme os recursos suportados pela versão.
- Exportação para impressão e outros formatos disponíveis no menu do aplicativo.

## Como usar

1. Abra **Design Smart** e crie um documento em **Arquivo > Novo**.
2. Defina o tamanho da página, margens e demais configurações do projeto.
3. Utilize as ferramentas de texto, imagem e formas para montar o layout.
4. Organize os elementos por páginas e ajuste tipografia, cores e alinhamentos.
5. Salve regularmente o arquivo de trabalho (`.designcraft`, formato técnico preservado por compatibilidade).
6. Para compartilhar ou imprimir, use a opção **Exportar** e escolha um formato disponível.

## Instalar no Windows e Linux

Os instaladores são produzidos pelos fluxos do GitHub Actions, após compilação bem-sucedida:

- **Windows x64:** arquivo `.msi`.
- **Linux x64:** `.AppImage`, `.deb`, `.rpm` e `.tar.gz`, conforme dependências e formatos gerados.

Instalação no Windows: execute o MSI. No Linux, use o gerenciador de pacotes da distribuição para DEB/RPM ou conceda permissão de execução à AppImage (`chmod +x arquivo.AppImage`).

## Compilar a partir do código

Requer toolchain Rust e as bibliotecas de compilação apropriadas ao sistema.

```bash
cargo build --release -p designcraft -p designcraft-cli
```

Para executar durante o desenvolvimento:

```bash
cargo run -p designcraft
```

O nome interno `designcraft` continua sendo utilizado por dependências e formatos de projeto para preservar compatibilidade.

## Atualizações

As novas versões podem ser publicadas em **GitHub Releases**. Publicar uma Release não atualiza automaticamente instalações existentes: o usuário deve instalar a versão nova, salvo quando houver um atualizador próprio implementado e testado.

## Licenças e atribuições

O projeto deriva de software de código aberto. Consulte os arquivos `LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE` e `ASSETS.md` para condições de uso, atribuições e licenças de componentes e imagens. Essas informações legais foram preservadas separadamente deste guia.
