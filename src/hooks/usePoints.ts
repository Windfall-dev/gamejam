import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { useCallback, useState } from "react";

import { useAuth } from "./useAuth";

export interface RankingUserProps {
  rank: string;
  name: string;
  points: string;
}

interface ApiResponse {
  ok: boolean;
  points: number;
  username: string;
}

export function usePoints() {
  const [user, setUser] = useState<RankingUserProps>({
    rank: "",
    name: "",
    points: "0",
  });

  const { accessToken, userId, username } = useAuth();

  const fetchPoints = useCallback(async () => {
    if (!accessToken || !userId) {
      throw new Error("No authentication data available");
      return;
    }
    try {
      const response = await fetch(
        `${process.env.NEXT_PUBLIC_FAST_API_URL}/api/auth/login`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            token: accessToken,
            user_id: userId,
            user_name: username || "",
          }),
        },
      );
      if (!response.ok) {
        throw new Error("Failed to fetch points");
      }
      const data: ApiResponse = await response.json();
      if (data.ok) {
        const points = Math.floor(data.points / LAMPORTS_PER_SOL);
        setUser({
          rank: "",
          name: data.username,
          points: points.toLocaleString("en"),
        });
      } else {
        throw new Error("Failed to get points from response");
      }
    } catch (err) {
      throw new Error("Unknown error occurred");
    }
  }, [accessToken, userId, username]);
  const addPoints = async (signature: string, userToken: string) => {
    if (!signature) {
      throw new Error("Failed to retrieve signature");
    }

    if (!userToken) {
      throw new Error("User token is required to add points.");
    }

    const requestBody = {
      signature,
    };

    try {
      const response = await fetch(
        `${process.env.NEXT_PUBLIC_FAST_API_URL}/api/point/deposit`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${accessToken}`,
          },
          body: JSON.stringify(requestBody),
        },
      );

      if (response.ok) {
        const data = await response.json();
        const new_point_balance = Math.floor(
          data.new_point_balance / LAMPORTS_PER_SOL,
        );
        setUser((prevUser) => ({
          ...prevUser,
          points: new_point_balance.toString(), // update points
        }));

        console.log(
          "Backend Return value is:",
          data.new_point_balance.toString(),
        );
        console.log("New Point Balance is: ", new_point_balance);
        return new_point_balance;
      } else if (response.status === 401) {
        const errorData = await response.json();
        console.error("Unauthorized:", errorData.message);
        throw new Error(errorData.message || "Unauthorized");
      } else {
        const errorData = await response.json();
        console.error("Error adding points:", errorData.message);
        throw new Error(errorData.message || "Failed to add points");
      }
    } catch (error: unknown) {
      if (error instanceof Error) {
        console.error("Error in handlePointAdd:", error.message);
        throw new Error(error.message);
      } else {
        console.error("An unexpected error occurred in handlePointAdd.");
        throw new Error("An unexpected error occurred.");
      }
    }
  };
  return { user, setUser, addPoints, fetchPoints };
}
