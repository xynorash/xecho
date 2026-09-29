# XEcho: A [Rust](https://rust-lang.org/) rewritten version of The [Echo](https://www.gnu.org/software/grub/manual/grub/html_node/echo.html) command from GNU.

Hey, there! Xynorash here!
<br><br>
This project is basically one which aims to reproduce how the [Echo](https://www.gnu.org/software/grub/manual/grub/html_node/echo.html) command from GNU/Linux behaves, as part of my personal arsenal and training materials.
<br><br>
I hope you'll enjoy my version! If there's anything you want to tell me (come on, don't be shy), just send me an email [here](mailto:nashtefison@gmail.com) or open an issue and I'll take care of you.
<br><br>
**Usage:** 
```bash
xecho [OPTIONS] <TEXT>
```
| Options | Actions |
| ----- | ----- |
| -n | Remove the newline at the end of the output |
| -h | Print help |
| --help | Print help |
| -V | Print version |
| --version | Print version |

**TEXT** is just the text you want to output.
<br><br>
**Example:** 
```bash
xecho -n Hello World
```
```bash
xecho "Hello World\n"
```
```bash
xecho Hello World
```
```bash
xecho -n Hello World
```

<br><br>
**Releases are [here](https://github.com/xynorash/xecho/releases)**
