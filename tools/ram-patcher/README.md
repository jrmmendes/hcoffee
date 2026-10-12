# largeaddress (port Linux)

Porte Linux em Rust do utilitário **LargeAddress** do Silent (parte do
SilentPatch), originalmente distribuído como `largeaddress.exe` para Windows.

## O que faz

Liga a flag **`IMAGE_FILE_LARGE_ADDRESS_AWARE`** (`0x0020`) no campo
`Characteristics` do cabeçalho COFF de executáveis 32-bit do GTA III / Vice
City / San Andreas.

Com a flag ativa, o jogo 32-bit passa a poder endereçar mais memória:

- **4 GB** em Windows/Linux **64-bit**;
- até **3 GB** em sistema **32-bit** com o switch `/3GB` habilitado
  (`bcdedit /set IncreaseUserVa 3072`).

Isso evita crashes por falta de RAM quando se usa muitos mods.

## Por que reescrever

O original é um `.exe` de Windows que roda "arrastando e soltando" o
executável do jogo sobre ele. Este porte:

- roda nativamente como **CLI Linux**;
- tem a mesma lógica de patch bit-a-bit (nenhuma outra alteração no arquivo);
- mantém backup `.bak` do original;
- reproduz as mensagens do utilitário original.

## Compilar

```sh
cargo build --release
# binário: target/release/ram-patcher
```

## Uso

```sh
ram-patcher <gta_exe>          # confirmação interativa [Y/n]
ram-patcher -y <gta_exe>       # responde "Y" automaticamente
ram-patcher -y -n <gta_exe>    # sem criar o backup .bak
```

Exemplos:

```sh
ram-patcher gta_sa.exe
ram-patcher -y ~/.steam/steam/steamapps/common/Grand\ Theft\ Auto\ San\ Andreas/gta_sa.exe
```

Saída típica:

```
Checking the input file...
Backing up an old EXE file...
Patching...
File successfully patched!
```

Se o arquivo já estiver patcheado:

```
ERROR: This file has a /LARGEADDRESSAWARE flag already!
```

## Como funciona

1. Abre o arquivo em modo binário e valida o cabeçalho DOS (`MZ`).
2. Lê `e_lfanew` (offset `0x3C`) e segue até a assinatura `PE\0\0`.
3. Localiza o campo `Characteristics` do COFF header (`PE + 4 + 18`).
4. Se o bit `0x0020` já estiver ligado, aborta com erro.
5. Faz backup para `<arquivo>.bak`.
6. Liga o bit e grava o arquivo.

O patch resultante altera **um único byte**: `Characteristics` passa, por
exemplo, de `0x0102` para `0x0122`.

## Estrutura

| Arquivo       | Descrição                                                                |
| ------------- | ------------------------------------------------------------------------ |
| `Cargo.toml`  | Metadados, dep `clap`, perfil release otimizado (`lto`, `strip`).        |
| `src/main.rs` | Toda a lógica: parsing PE, checagem/patch da flag, backup, CLI e testes. |

## Testes

```sh
cargo test
```

Cobrem: localização do campo `Characteristics`, rejeição de arquivo sem `MZ`,
rejeição de assinatura `PE` inválida, detecção de flag já presente e geração do
nome do backup.

## Créditos

Ferramenta original **LargeAddress** por **Silent** (SilentPatch, 1.1).
Distribuída por MixMods.com.br. Este porte apenas reproduz o comportamento em
Rust para Linux.
