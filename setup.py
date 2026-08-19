from setuptools import setup

setup(
    name="zap-lang",
    version="0.2.0",
    description="Zap: a beginner-friendly Web and AI programming language",
    py_modules=["zap"],
    python_requires=">=3.9",
    entry_points={"console_scripts": ["zap=zap:main"]},
)
