# Zap v0.5.0

Zap v0.5.0 သည် native runtime၊ project testing workflow နှင့် documentation ကို တိုးချဲ့ထားသော release ဖြစ်သည်။

## Highlights

- `zap test` project-level test runner
- `tests/` အောက်ရှိ nested directories များမှ `*_test.zp` files များကို recursive discovery ပြုလုပ်နိုင်ခြင်း
- `zap init` ဖြင့် `main.zp`၊ `zap.toml` နှင့် starter `tests/smoke_test.zp` ကို ဖန်တီးနိုင်ခြင်း
- Standard runtime helpers နှင့် collection/text/numeric utilities
- `docs/` အောက်တွင် language၊ syntax၊ usage၊ package နှင့် ecosystem guides များကို စနစ်တကျခွဲထားခြင်း
- `examples/` အောက်တွင် run နိုင်သော beginner နှင့် practical programs များ ပါဝင်ခြင်း
- Windows၊ Linux နှင့် macOS ARM64 native binary release archives
- SHA-256 checksum files ဖြင့် archive verification

## Quick Start

Windows တွင် `zap-windows-x86_64.zip` ကို download လုပ်ပြီး extract လုပ်ပါ။ ထို့နောက် direct executable ကို run နိုင်ပါသည်။

```bat
cd zap-0.5.0
bin\zap.exe --version
bin\zap.exe examples\hello.zp
```

Global `zap` command သုံးရန် extract လုပ်ထားသော folder ထဲမှ—

```bat
install_windows.bat
```

ကို run လုပ်ပြီး Command Prompt အသစ်ဖွင့်ပါ။

Linux/macOS တွင် archive ကို extract လုပ်ပြီး—

```bash
cd zap-0.5.0
./bin/zap --version
./bin/zap examples/hello.zp
```

သို့မဟုတ် `install.sh` ကို run လုပ်နိုင်ပါသည်။

## Project Testing

```bash
zap init hello-zap
cd hello-zap
zap check
zap test
zap main.zp
```

Test files များကို `tests/` directory အောက်တွင် `*_test.zp` naming ဖြင့် ထားပါ။ Test runner သည် nested test directories များကိုပါ ရှာဖွေပြီး failure ဖြစ်ပါက non-zero exit code ပြန်ပေးသည်။

## Documentation

အသေးစိတ်အသုံးပြုနည်းအတွက် [Language Guide](LANGUAGE_GUIDE.md)၊ [Syntax Guide](SYNTAX_GUIDE.md)၊ [Usage Guide](USAGE.md) နှင့် [Package Guide](PACKAGE.md) ကို ဖတ်ရှုပါ။

## Verification

Release မတင်မီ native Cargo tests၊ Zap examples၊ `zap init` scaffold၊ `zap check`၊ `zap test` နှင့် archive checksum များကို စစ်ဆေးရမည်။

## Repository

- Repository: https://github.com/hidecard/zap
- Release tag: `v0.5.0`
- File extension: `.zp`
- Project manifest: `zap.toml`

## License

Zap repository ၏ လက်ရှိ license နှင့် contribution terms များအတိုင်း အသုံးပြုနိုင်သည်။
