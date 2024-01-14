(program
  1.0.0
  [
    (lam
      i_0
      [
        (lam
          i_0
          [
            (lam
              i_0
              [
                (lam
                  i_0
                  [
                    (lam
                      i_0
                      (lam
                        i_0
                        (lam
                          i_0
                          (lam
                            i_0
                            (lam
                              i_0
                              [
                                (lam
                                  i_0
                                  [
                                    [ [ i_1 i_1 ] i_5 ]
                                    [
                                      (builtin unMapData)
                                      [
                                        i_9
                                        [
                                          i_10
                                          [
                                            i_10
                                            [
                                              i_10
                                              [
                                                i_10
                                                [
                                                  i_10
                                                  [
                                                    i_10
                                                    [
                                                      i_8
                                                      [
                                                        (builtin unConstrData)
                                                        [
                                                          i_9
                                                          [
                                                            i_8
                                                            [
                                                              (builtin
                                                                unConstrData
                                                              )
                                                              i_2
                                                            ]
                                                          ]
                                                        ]
                                                      ]
                                                    ]
                                                  ]
                                                ]
                                              ]
                                            ]
                                          ]
                                        ]
                                      ]
                                    ]
                                  ]
                                )
                                (lam
                                  i_0
                                  (lam
                                    i_0
                                    (lam
                                      i_0
                                      (force
                                        [
                                          [
                                            [
                                              i_8
                                              [
                                                [
                                                  (builtin equalsData)
                                                  [ i_9 [ i_11 i_1 ] ]
                                                ]
                                                i_2
                                              ]
                                            ]
                                            (delay (con unit ()))
                                          ]
                                          (delay
                                            [ [ [ i_3 i_3 ] i_2 ] [ i_12 i_1 ] ]
                                          )
                                        ]
                                      )
                                    )
                                  )
                                )
                              ]
                            )
                          )
                        )
                      )
                    )
                    (force (builtin ifThenElse))
                  ]
                )
                (force (force (builtin fstPair)))
              ]
            )
            (force (force (builtin sndPair)))
          ]
        )
        (force (builtin headList))
      ]
    )
    (force (builtin tailList))
  ]
)