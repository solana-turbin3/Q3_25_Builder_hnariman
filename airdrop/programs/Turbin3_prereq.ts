export type Turbin3Prereq = {
  version: "0.1.0";
  name: "Turbin3_prereq";
  addresses: string[];
  message: string;
};

export type IDL = {
  address: "TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM";
  metadata: {
    name: "q3_pre_reqs_rs";
    version: "0.1.0";
    spec: "0.1.0";
    description: "Created with Anchor";
  };
  instructions: [
    {
      name: "close";
      discriminator: number[];
      accounts: [
        { name: "user"; writable: true },
        {
          name: "account";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: number[];
              },
              {
                kind: "account";
                path: "user";
              },
            ];
          };
        },
        {
          name: "system_program";
          address: "11111111111111111111111111111111";
        },
      ];
      args: [];
    },
    {
      name: "create_collection";
      discriminator: number[];
      accounts: [
        { name: "creator"; writable: true; signer: true },
        { name: "collection"; writable: true; signer: true },
        {
          name: "authority";
          pda: {
            seeds: [
              {
                kind: "const";
                value: number[];
              },
              {
                kind: "account";
                path: "collection";
              },
            ];
          };
        },
        {
          name: "mpl_core_program";
          address: "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d";
        },
        {
          name: "system_program";
          address: "11111111111111111111111111111111";
        },
      ];
      args: [];
    },
    {
      name: "initialize";
      discriminator: number[];
      accounts: [
        {
          name: "user";
          writable: true;
          signer: true;
        },
        {
          name: "account";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: number[];
              },
              {
                kind: "account";
                path: "user";
              },
            ];
          };
        },
        {
          name: "system_program";
          address: "11111111111111111111111111111111";
        },
      ];
      args: [
        {
          name: "github";
          type: "string";
        },
      ];
    },
    {
      name: "submit_rs";
      discriminator: number[];
      accounts: [
        {
          name: "user";
          writable: true;
          signer: true;
        },
        {
          name: "account";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: number[];
              },
              {
                kind: "account";
                path: "user";
              },
            ];
          };
        },
        {
          name: "mint";
          writable: true;
          signer: true;
        },
        {
          name: "collection";
          writable: true;
        },
        {
          name: "authority";
          pda: {
            seeds: [
              {
                kind: "const";
                value: number[];
              },
              {
                kind: "account";
                path: "collection";
              },
            ];
          };
        },
        {
          name: "mpl_core_program";
          address: "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d";
        },
        {
          name: "system_program";
          address: "11111111111111111111111111111111";
        },
      ];
      args: [];
    },
    {
      name: "submitTs";
      discriminator: number[];
      accounts: [
        {
          name: "user";
          writable: true;
          signer: true;
        },
        {
          name: "account";
          writable: true;
          pda: {
            seeds: [
              {
                kind: "const";
                value: number[];
              },
              {
                kind: "account";
                path: "user";
              },
            ];
          };
        },
        {
          name: "mint";
          writable: true;
          signer: true;
        },
        {
          name: "collection";
          writable: true;
        },
        {
          name: "authority";
          pda: {
            seeds: [
              {
                kind: "const";
                value: number[];
              },
              {
                kind: "account";
                path: "collection";
              },
            ];
          };
        },
        {
          name: "mpl_core_program";
          address: "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d";
        },
        {
          name: "system_program";
          address: "11111111111111111111111111111111";
        },
      ];
      args: [];
    },
    {
      name: "update";
      discriminator: number[];
      accounts: [
        {
          name: "user";
          writable: true;
          signer: true;
        },
        {
          name: "account";
          pda: {
            seeds: [
              {
                kind: "const";
                value: number[];
              },
              {
                kind: "account";
                path: "user";
              },
            ];
          };
        },
        {
          name: "system_program";
          address: "11111111111111111111111111111111";
        },
      ];
      args: [
        {
          name: "github";
          type: "string";
        },
      ];
    },
  ];
  accounts: [
    {
      name: "ApplicationAccount";
      discriminator: number[];
    },
  ];
  errors: [
    {
      code: 6000;
      name: "PreReqTsNotCompleted";
      msg: "TS submission not completed.";
    },
    {
      code: 6001;
      name: "PreReqTsAlreadyCompleted";
      msg: "TS submission already completed.";
    },
    {
      code: 6002;
      name: "PreReqRsAlreadyCompleted";
      msg: "Rust submission already completed.";
    },
    {
      code: 6003;
      name: "PreReqRsNotInTimeWindow";
      msg: "Submission not allowed.";
    },
  ];
  types: [
    {
      name: "ApplicationAccount";
      type: {
        kind: "struct";
        fields: [
          {
            name: "user";
            type: "pubkey";
          },
          {
            name: "bump";
            type: "u8";
          },
          {
            name: "pre_req_ts";
            type: "bool";
          },
          {
            name: "pre_req_rs";
            type: "bool";
          },
          {
            name: "github";
            type: "string";
          },
        ];
      };
    },
  ];
};
