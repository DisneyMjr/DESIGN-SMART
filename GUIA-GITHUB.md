# Publicar Design Smart no GitHub

Crie um repositório vazio `DisneyMjr/DESIGN-SMART` e envie esta pasta com Git.

No GitHub, abra **Actions** e escolha **Gerar MSI Design Smart**, **Gerar Linux Design Smart** ou **Publicar Design Smart**. Se usar **Publicar**, informe a versão de `[workspace.package]` do Cargo.toml, com pontos (ex.: `0.8.0`). O fluxo publica os artefatos em **Releases** após as duas compilações passarem.

Os workflows foram preparados, mas não foram validados em runners Windows/Linux nesta entrega. Ajustes de dependências podem ser necessários. A publicação da Release não atualiza automaticamente programas já instalados.

## Enviar alterações a um repositório já clonado

Copie os arquivos, mantendo `.git`, e execute:

```powershell
git add .
git commit -m "Design Smart - identidade e instaladores"
git push origin main
```

## Histórico e licenças

Mantenha os arquivos `NOTICE`, `LICENSE-MIT`, `LICENSE-APACHE` e `ASSETS.md` ao redistribuir.
