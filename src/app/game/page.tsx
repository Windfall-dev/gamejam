import Image from "next/image";
import Link from "next/link";
import React from "react";

import { Button } from "@/components/ui/button";

const GamePage: React.FC = () => (
  <main className="mx-auto bg-transparent">
    <div className="relative aspect-square overflow-hidden">
      <Image
        src="/bonk_image.png"
        fill={true}
        alt="Game Thumb"
        objectFit="contain"
        className="scale-[1.12]"
      />
    </div>
    <div className="relative top-[-70px]">
      <div className="flex flex-col items-center">
        <div className="flex w-full justify-center bg-gradient-to-t from-white to-white/0">
          <Image
            src="/bonk_top_title.png"
            height={120}
            width={120}
            alt="game icon"
            className="rounded-xl"
          />
        </div>
        <div className="px-5 py-4 text-center">
          <p className="text-body-bold text-muted-foreground">Bonk</p>
          <h3>Bonk the Digger</h3>
          {/* <p className="body text-muted-foreground">
            In Beat Pet, cute pets will test your sense of <br /> rhythm and
            luck!
          </p> */}
        </div>
        <Link href="../../game/play">
          <Button size="L" className="w-[315px] rounded-xl px-[30px]">
            <h2>PLAY</h2>
          </Button>
        </Link>
      </div>
    </div>
  </main>
);

export default GamePage;
