import React, { useState, ChangeEvent, useRef, useEffect } from "react";
import { signIn } from "~/lib/auth";
import { API_URL, API_VERSION, DOMAIN } from "~/lib/config";

interface TradingInterfaceProps {
  symbol: string;
  authenticated: boolean;
  currentMarketPrice?: number; // Optional market price for calculations
}

interface OrderRequest {
  product_id: string;
  side: string;
  type: string;
  size: number;
  price?: number;
}

type TradeSide = "buy" | "sell";
type OrderType = "limit" | "market";

const SideToggleButton: React.FC<{
  label: string;
  isActive: boolean;
  onClick: () => void;
  activeClass: string;
}> = ({ label, isActive, onClick, activeClass }) => (
  <button
    className={`flex-1 py-2 text-center font-medium transition-colors h-12 ${
      isActive ? activeClass : "text-gray-400 bg-[rgb(40,43,49)]"
    }`}
    onClick={onClick}
  >
    {label}
  </button>
);

const OrderTypeButton: React.FC<{
  type: OrderType;
  isActive: boolean;
  onClick: () => void;
}> = ({ type, isActive, onClick }) => (
  <button
    className={`px-3 py-2 rounded-full text-xs font-medium ${
      isActive
        ? "bg-[rgb(0,16,51)] text-[rgb(87,139,250)]"
        : "bg-transparent text-white"
    }`}
    onClick={onClick}
  >
    {type.charAt(0).toUpperCase() + type.slice(1)}
  </button>
);

const AdjustmentButton: React.FC<{
  label: string;
  onClick: () => void;
}> = ({ label, onClick }) => (
  <button
    className="bg-[#1E1E1E] border border-[#333] rounded-[4px] py-1 text-center text-gray-400 text-[11px]"
    onClick={onClick}
  >
    {label}
  </button>
);

interface LimitPriceInputProps {
  limitPrice: string;
  onLimitPriceChange: (value: string) => void;
  isActive: boolean;
}

const LimitPriceInput: React.FC<LimitPriceInputProps> = ({
  limitPrice,
  onLimitPriceChange,
  isActive,
}) => {
  const handleInputChange = (e: ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value;
    // Only allow numeric input with optional decimal point
    if (/^\d*\.?\d{0,2}$/.test(value)) {
      onLimitPriceChange(value);
    }
  };

  return (
    <div className="mb-2">
      <label className="block">
        <div
          className={`outline-none flex items-center bg-[#1E1E1E] border rounded-[4px] h-8 px-[10px] py-[8px] text-[11px] cursor-text ${
            isActive
              ? "border-blue-500 outline-2 outline-blue-500"
              : "border-[#333]"
          }`}
          onClick={(e) => e.currentTarget.querySelector("input")?.focus()}
        >
          <span className="text-white w-2/3 flex-1 font-medium">
            Limit price
          </span>
          <div className="flex-2 w-1/3">
            <input
              value={limitPrice}
              onChange={handleInputChange}
              placeholder="0"
              type="text"
              inputMode="decimal"
              pattern="[0-9]*"
              className="placeholder:text-gray-400 text-right text-white bg-transparent w-full outline-none"
            />
          </div>
        </div>
      </label>
    </div>
  );
};

const Divider = () => <div className="h-px bg-[#333] w-full my-2"></div>;

export default function TradingInterface({
  symbol,
  authenticated,
  currentMarketPrice = 0,
}: TradingInterfaceProps) {
  // State management
  const [side, setSide] = useState<TradeSide>("buy");
  const [orderType, setOrderType] = useState<OrderType>("limit");
  const [limitPrice, setLimitPrice] = useState("0");
  const [isLimitPriceInputActive, setIsLimitPriceInputActive] = useState(false);
  const limitPriceInputRef = useRef<HTMLInputElement>(null);
  const [orderSize] = useState(1); // Simplified for now

  // Calculate total order value
  const calculateOrderTotal = () => {
    let price = 0;
    if (limitPrice) {
      price = parseFloat(limitPrice);
    }
    return orderType === "market"
      ? currentMarketPrice * orderSize
      : price * orderSize;
  };

  // Price adjustment helpers
  const adjustPrice = (percentage: number) => {
    const currentPrice = parseFloat(limitPrice) || currentMarketPrice;
    const adjustment =
      side === "buy"
        ? currentPrice * (1 - percentage / 100)
        : currentPrice * (1 + percentage / 100);

    setLimitPrice(adjustment.toFixed(0));
  };

  const handleMidPrice = () => {
    // For now, using current market price as mid price
    setLimitPrice(currentMarketPrice.toFixed(0));
  };

  const handleBidAskPrice = () => {
    // Simplified: using market price
    // In a real implementation, this would fetch bid/ask from an API
    setLimitPrice(currentMarketPrice.toFixed(0));
  };

  async function createOrder() {
    // Prevent order creation if limit price is 0
    if (orderType === "limit" && limitPrice === "0") return;

    let body: OrderRequest = {
      product_id: `${symbol}`,
      side: `${side}`,
      type: `${orderType}`,
      size: orderSize,
    };

    if (limitPrice && orderType === "limit") {
      body.price = Number(limitPrice);
    }

    const response = await fetch(`${API_URL}/${API_VERSION}/orders`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      credentials: "include",
      body: JSON.stringify(body),
    });
  }

  const AuthButtons = () => {
    const isOrderButtonDisabled =
      (orderType === "limit" && limitPrice === "0") || !authenticated;

    return (
      <div className="space-y-2">
        {authenticated ? (
          <button
            className={`w-full py-2 rounded-full text-sm font-medium ${
              isOrderButtonDisabled
                ? "bg-gray-500/50 text-gray-500 cursor-not-allowed"
                : "bg-blue-400 text-blue-950"
            }`}
            onClick={() => createOrder()}
            disabled={isOrderButtonDisabled}
          >
            {side === "buy" ? "Buy" : "Sell"}
          </button>
        ) : (
          <button
            className="w-full bg-blue-400 text-blue-950 py-2 rounded-full text-sm font-medium"
            onClick={() => signIn(`${DOMAIN}/trade/${symbol}`)}
          >
            Sign in
          </button>
        )}
      </div>
    );
  };

  // Styling constants
  const STYLES = {
    buyActiveClass: "border-t-2 border-t-[#4CAF50] text-[#4CAF50]",
    sellActiveClass: "border-t-2 border-t-[#F44336] text-[#F44336]",
    inactiveButtonClass: "text-gray-400 bg-[rgb(40,43,49)]",
    orderTypeActiveClass: "bg-[rgb(0,16,51)] text-[rgb(87,139,250)]",
    orderTypeInactiveClass: "bg-transparent text-white",
  };

  // Render methods for different sections
  const SideToggle = () => (
    <div className="flex w-full">
      <SideToggleButton
        label="Buy"
        isActive={side === "buy"}
        onClick={() => setSide("buy")}
        activeClass={STYLES.buyActiveClass}
      />
      <SideToggleButton
        label="Sell"
        isActive={side === "sell"}
        onClick={() => setSide("sell")}
        activeClass={STYLES.sellActiveClass}
      />
    </div>
  );

  const OrderTypeButtons = () => {
    const orderTypes: OrderType[] = ["limit", "market"];

    return (
      <div className="flex gap-2 mb-4">
        {orderTypes.map((type) => (
          <OrderTypeButton
            key={type}
            type={type}
            isActive={orderType === type}
            onClick={() => setOrderType(type)}
          />
        ))}
      </div>
    );
  };

  const PriceAdjustmentButtons = () => {
    const priceAdjustments = [
      {
        label: "Mid",
        action: handleMidPrice,
      },
      {
        label: side === "buy" ? "Bid" : "Ask",
        action: handleBidAskPrice,
      },
      {
        label: side === "buy" ? "1%↓" : "1%↑",
        action: () => adjustPrice(1),
      },
      {
        label: side === "buy" ? "5%↓" : "5%↑",
        action: () => adjustPrice(5),
      },
    ];

    return (
      <div className="grid grid-cols-4 gap-1 mb-4">
        {priceAdjustments.map((adjustment, index) => (
          <AdjustmentButton
            key={index}
            label={adjustment.label}
            onClick={adjustment.action}
          />
        ))}
      </div>
    );
  };

  return (
    <div className={`flex flex-col bg-[#121212] text-white h-full`}>
      <SideToggle />

      <div className={`p-4 relative`}>
        <OrderTypeButtons />

        {/* Conditionally render limit price and price adjustment buttons */}
        {orderType === "limit" && (
          <>
            <LimitPriceInput
              limitPrice={limitPrice}
              onLimitPriceChange={setLimitPrice}
              isActive={isLimitPriceInputActive}
            />
            <PriceAdjustmentButtons />
          </>
        )}

        <Divider />
        <div className="space-y-1 mb-4 text-xs">
          <div className="flex justify-between">
            <span className="text-gray-400">Total</span>
            <span className="text-white">
              {calculateOrderTotal().toFixed(0)}
            </span>
          </div>
        </div>
        <AuthButtons />
      </div>
    </div>
  );
}
