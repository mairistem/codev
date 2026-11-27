from pantry.stock import Stock

stock = Stock()
stock.add("coffee", 3)
for item, count in stock.low(threshold=5):
    print(f"Running low: {item} ({count} left)")
