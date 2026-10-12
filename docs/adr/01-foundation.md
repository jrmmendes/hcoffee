# ADR 01: foundation

## Contexto

o hcoffee é um utilitário de linha de comando escrito em node 26 + typescript. O
objetivo é simplificar a manipulação do GTA SA em linux, incluindo setup inicial
(downgrade para v1.0 e fixes básicos), instalação de gerenciamento de mods e
rollback seguro do game.

O seguinte uso é esperado:

```bash
$ hcoffee init -y
# downgrade para 1.0, ajuste no limite de RAM, configurações de versão
# e launch options da steam, bootstrap de game.json (baseado no package.json do
# node) e game-lock.json (lockfile para garantir reprodução exata), modloader, CLEO,
# ajustadores de limite, etc

$ hcoffee add gta-iv-hud@v1
# instala v1 do gta-iv-hud. É experado que o pacote gta-iv-hud.hf exista em um
# registry compatível, com protocolo a ser definido.

$ hcoffee remove gta-iv-hud@v1 # remove totalmente e volta ao padrão
```
