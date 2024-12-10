import Image from "next/image";
import React, { useEffect, useState } from "react";

import { InfoDouble } from "@/components/Info";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogTrigger,
} from "@/components/ui/dialog";
import { useAuth } from "@/hooks/useAuth";
import { RankingUserProps } from "@/hooks/usePoints";

import { Button } from "./ui/button";
import { Card, CardHeader, CardTitle } from "./ui/card";

function SeasonResult() {
  const [user, setUser] = useState<RankingUserProps>({
    rank: "",
    name: "",
    points: "",
  });
  const context = useAuth();
  const userId = context.userId.slice(0, 4) + ".." + context.userId.slice(-4);

  useEffect(() => {
    const fetchRankings = async () => {
      try {
        const response = await fetch(
          `${process.env.NEXT_PUBLIC_FAST_API_URL}/api/rankings`,
          {
            method: "GET",
          },
        );
        const data = await response.json();
        console.log("Data", data);
        const user = data.rankings.find(
          (user: RankingUserProps) => user.name === userId,
        );
        console.log("User", user);
        setUser(user);
      } catch (error) {
        console.error("Error fetching rankings:", error);
      }
    };
    fetchRankings();
  }, [userId]);

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Image
          src="/windfall_logo_h.png"
          alt="Windfall"
          width={150}
          height={40}
        />
      </DialogTrigger>
      <DialogContent className="w-full max-w-[350px] overflow-hidden rounded-2xl p-0">
        <Card className="w-full overflow-hidden border-0">
          <CardHeader className="relative h-[186px] p-0">
            <Image
              src="/image_season_result.png"
              alt="Windfall"
              layout="fill"
              objectFit="cover"
            />
            <CardTitle className="z-1000 absolute bottom-4 w-full px-[10px] text-center">
              <h2>The season has ended.</h2>
            </CardTitle>
          </CardHeader>
          <div className="mx-auto mb-10 mt-6 flex flex-col items-center justify-center space-y-6">
            <div className="w-full max-w-[275px] space-y-[6px]">
              <p className="text-body-title text-center text-wf-orange">
                Your Results
              </p>
              <InfoDouble
                text1a="Prize"
                text1b={user?.points || ""}
                image1="/sol.png"
                alt1="Sol"
                text2a="Final Ranking"
                text2b={user?.rank || ""}
                image2="/icon_ranking.png"
                alt2="Ranking"
                optionStyle={["text-center", "justify-center"]}
                isSol={true}
              />
            </div>

            <DialogClose asChild>
              <Button size="L">
                <h2 className="px-8 py-3">OK</h2>
              </Button>
            </DialogClose>
          </div>
        </Card>
      </DialogContent>
    </Dialog>
  );
}

export default SeasonResult;
