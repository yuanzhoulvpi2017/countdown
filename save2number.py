import matplotlib.pyplot as plt
import os

number_dir = "NumberSimple"

os.makedirs(name=number_dir, exist_ok=True)


def gennumber(number):
    fig, ax = plt.subplots(figsize=(2, 3))
    with plt.xkcd():
        ax.text(-0.05, 0.1, f"{number}", weight="bold", fontsize=200)
        ax.axis('off')
        plt.show()
        fig.savefig(f"{number_dir}/{number}.png")


def gensymbol(symbol, symbol_name="maohao"):
    fig, ax = plt.subplots(figsize=(2, 3))
    with plt.xkcd():
        ax.text(-0.05, 0, f"{symbol}", weight="bold", fontsize=200)
        ax.axis('off')
        plt.show()
        fig.savefig(f"{number_dir}/{symbol_name}.png")

def genlabel(label):
    fig, ax = plt.subplots(figsize=(2, 1))
    with plt.xkcd():
        ax.text(0, 0.2, f"{label}", weight="bold", fontsize=60)
        ax.axis('off')
        plt.show()
        fig.savefig(f"{number_dir}/{label}.png")


if __name__ == '__main__':
    for num in range(0, 10):
        gennumber(num)

    #
    gensymbol(symbol=":")
    [genlabel(i) for i in ["day", "hou", "min", "sec"]]
